# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust API client library for The Movie Database (TMDB) API, generated from a custom OpenAPI specification (tmdb_oas.json) using OpenAPI Generator. The library provides comprehensive access to TMDB's v3 API endpoints.

## Build and Test Commands

### Build
```bash
# Default (blocking + native-tls)
cargo build

# Async only (smaller binary)
cargo build --no-default-features --features "async,native-tls"

# Both blocking and async
cargo build --features "blocking,async"
```

### Run Tests
Tests require a TMDB API key set in the `TMDB_API_KEY` environment variable:
```bash
export TMDB_API_KEY="your_api_key_here"

# Run all tests (requires blocking feature, which is default)
cargo test

# Run tests without daily-exports tests
cargo test --no-default-features --features "blocking,native-tls"

# Run with daily-exports tests
cargo test --features "blocking,daily-exports"
```

To run a single test:
```bash
cargo test <test_name>
```

**Note**: Most tests require the `blocking` feature (enabled by default). The `external_id_tests` require the `daily-exports` feature.

### Run Examples
```bash
# Async example
export TMDB_API_KEY="your_api_key_here"
cargo run --example async_example --no-default-features --features "async,native-tls"
```

### Documentation
Generate and view local documentation:
```bash
cargo doc --open
```

### Lint
```bash
cargo clippy
```

### Dependency Updates
```bash
cargo upgrade  # Updates dependencies
```

## Architecture

### Client Initialization Pattern

The library provides **two client implementations**:

#### 1. Blocking (Synchronous) Client
- Uses `reqwest::blocking::Client` for synchronous HTTP requests
- Located in `src/apis/`
- Entry point: `APIClient` from `tmdb_client::apis::client`
- Configuration wrapped in `Rc` for shared state
- Initialization methods:
  - `APIClient::new(configuration)` - With custom configuration
  - `APIClient::new_with_api_key(api_key)` - Direct API key
  - `APIClient::new_from_env()` - Reads from `TMDB_API_KEY` env var

#### 2. Async Client (NEW in 1.9.0)
- Uses `reqwest::Client` for asynchronous HTTP requests with tokio
- Located in `src/async_apis/`
- Entry point: `AsyncAPIClient` from `tmdb_client::async_apis`
- Configuration wrapped in `Arc` for thread-safe shared state
- All methods are `async` and must be `.await`ed
- Requires `#[async_trait]` for trait definitions (handled automatically)
- Initialization methods (same as blocking):
  - `AsyncAPIClient::new(configuration)`
  - `AsyncAPIClient::new_with_api_key(api_key)`
  - `AsyncAPIClient::new_from_env()`

Both clients share the same models (`src/models/`) and error types.

### Module Structure
- `src/apis/` - API endpoint implementations (one file per TMDB API category)
  - Each API has a trait (e.g., `MoviesApi`) and client struct (e.g., `MoviesApiClient`)
  - `client.rs` - Main `APIClient` facade that aggregates all API clients
  - `configuration.rs` - HTTP client configuration (base URL, user agent, API key)
- `src/models/` - Data models for requests/responses (auto-generated from OpenAPI spec)
- `src/files/` - Helper functions for downloading and parsing TMDB daily export files (requires `daily-exports` feature)
  - `exports.rs` - Functions to fetch gzipped JSON exports for movies, TV shows, people, etc.
  - `models.rs` - Models for exported ID data structures

### Error Handling
Unified error type in `src/lib.rs` using `thiserror`:
- `Error::Reqwest` - HTTP client errors
- `Error::Serde` - JSON serialization/deserialization errors
- `Error::Io` - I/O errors (file operations)

### Key Dependencies
- `reqwest` (0.12) - HTTP client with both blocking and async APIs, JSON and gzip support
- `tokio` (1.47+, optional) - Async runtime for async features
- `async-trait` (0.1, optional) - Enables async traits for async API
- `serde` / `serde_json` - Serialization
- `serde_with` (3.14+) - Handles TMDB's inconsistent API (fields that can be String or Int)
- `chrono` - Date/time handling for releases and daily exports
- `flate2` (optional, requires `daily-exports` feature) - Decompress gzipped daily export files

### Important Implementation Details

**TMDB API Inconsistencies**: The API sometimes returns inconsistent types for the same field (e.g., a field might be a String in one response and an Int in another). The library uses the `serde_with` crate to handle these cases gracefully. If you encounter deserialization errors, check if a field needs flexible type handling.

**Blocking vs Async**:
- The blocking client uses `reqwest::blocking::Client` - all API calls are synchronous
- The async client uses `reqwest::Client` - all API calls return `Future`s that must be `.await`ed
- Both clients share the same models and can coexist in the same project
- The async client is thread-safe (`Send + Sync`) and uses `Arc` instead of `Rc`

**Optional Append-To-Response**: Many TMDB endpoints support an `append_to_response` parameter that allows fetching related data in a single request (e.g., credits, videos, images). When this parameter is not provided, the corresponding fields in response models will be `None`.

**Daily Export Files**: The `files` module (requires `daily-exports` feature) provides utilities to download TMDB's daily export files (http://files.tmdb.org/p/exports). These are gzipped newline-delimited JSON files containing entity IDs and basic metadata. This feature is optional and must be explicitly enabled in `Cargo.toml`.

## Testing Strategy

Tests are integration tests in the `tests/` directory that make real API calls to TMDB. Each test file corresponds to an API category (e.g., `movie_api_tests.rs`, `search_api_tests.rs`).

Tests use `APIClient::new_from_env()` which requires `TMDB_API_KEY` environment variable.

## Code Generation

Most of the codebase is auto-generated from `tmdb_oas.json` using OpenAPI Generator. When modifying:
- **DO NOT** edit generated model files or API client implementations directly
- **DO** update `tmdb_oas.json` and regenerate if API changes are needed
- **CAN** edit: `src/lib.rs`, `src/files/`, test files, and `APIClient` convenience methods

The `.openapi-generator-ignore` file controls which files are protected from regeneration.
