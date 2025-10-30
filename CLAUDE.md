# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust API client library for The Movie Database (TMDB) API, generated from a custom OpenAPI specification (tmdb_oas.json) using OpenAPI Generator. The library provides comprehensive access to TMDB's v3 API endpoints.

## Build and Test Commands

### Build
```bash
cargo build
```

### Run Tests
Tests require a TMDB API key set in the `TMDB_API_KEY` environment variable:
```bash
export TMDB_API_KEY="your_api_key_here"
cargo test
```

To run a single test:
```bash
cargo test <test_name>
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
The library uses a centralized `APIClient` that manages all API endpoint clients. Three initialization methods are available:

1. `APIClient::new(configuration)` - With custom configuration
2. `APIClient::new_with_api_key(api_key)` - Direct API key
3. `APIClient::new_from_env()` - Reads from `TMDB_API_KEY` env var

All API clients share a single `Configuration` wrapped in `Rc` for efficient cloning and shared state management.

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
- `reqwest` (0.12) - HTTP client with blocking API, JSON and gzip support
- `serde` / `serde_json` - Serialization
- `serde_with` (3.14+) - Handles TMDB's inconsistent API (fields that can be String or Int)
- `chrono` - Date/time handling for releases and daily exports
- `flate2` (optional, requires `daily-exports` feature) - Decompress gzipped daily export files

### Important Implementation Details

**TMDB API Inconsistencies**: The API sometimes returns inconsistent types for the same field (e.g., a field might be a String in one response and an Int in another). The library uses the `serde_with` crate to handle these cases gracefully. If you encounter deserialization errors, check if a field needs flexible type handling.

**Blocking Client**: This library uses `reqwest::blocking::Client`. All API calls are synchronous/blocking.

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
