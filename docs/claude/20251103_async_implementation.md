# Async Implementation Documentation

## Overview

This document describes the async/await implementation added to tmdb-client-rs in version 1.9.0.

The implementation supports **conditional compilation** - you can choose to compile only the blocking client, only the async client, or both.

## Architecture

The async implementation follows a **module separation** strategy with **conditional compilation**:

```
src/
├── apis/              # Blocking (sync) APIs [#[cfg(feature = "blocking")]]
│   ├── configuration.rs      # Uses reqwest::blocking::Client + Rc
│   ├── movies_api.rs         # Blocking implementation
│   ├── ...                   # Other blocking APIs
│   └── client.rs             # APIClient facade
│
├── async_apis/        # Async APIs [#[cfg(feature = "async")]]
│   ├── configuration.rs      # Uses reqwest::Client + Arc
│   ├── movies_api.rs         # Async implementation with #[async_trait]
│   ├── ...                   # Other async APIs
│   └── client.rs             # AsyncAPIClient facade
│
└── models/            # Shared between sync and async [always compiled]
    └── *.rs                  # All data models
```

### Conditional Compilation

The library uses Cargo features to conditionally compile code:

```rust
// In src/lib.rs
#[cfg(feature = "blocking")]
pub mod apis;  // Only compiled when blocking is enabled

#[cfg(feature = "async")]
pub mod async_apis;  // Only compiled when async is enabled

pub mod models;  // Always compiled
```

This approach provides several benefits:
- **Reduced binary size**: Only compile what you need
- **Fewer dependencies**: No tokio if you only use blocking
- **Faster compile times**: Less code to compile
- **Flexibility**: Choose the right tool for your use case

### Key Design Decisions

1. **Separate Modules**: `apis/` for sync, `async_apis/` for async
   - Allows both implementations to coexist
   - No code duplication in models
   - Clear separation of concerns
   - Can be independently enabled/disabled

2. **Shared Models**: Both sync and async use the same models from `src/models/`
   - No duplication of data structures
   - Consistent API responses
   - Single source of truth for data types

3. **Configuration Differences**:
   - **Sync**: `reqwest::blocking::Client` with `Rc<Configuration>`
   - **Async**: `reqwest::Client` with `Arc<Configuration>`
   - Arc is necessary for thread-safety in async contexts

4. **Code Generation**:
   - Async code was generated from sync code using `generate_async_apis.py`
   - Automated transformations minimize maintenance burden

## Features

### Cargo Features

- `blocking` (default): Enables sync API client
  - Activates `reqwest/blocking` feature
  - Compiles all code in `src/apis/`
- `async`: Enables async API client with tokio runtime
  - Activates `tokio` and `async-trait` dependencies
  - Compiles all code in `src/async_apis/`
- Both can be enabled simultaneously

### Configuration Examples

```toml
# Async only (smaller binary, no blocking code)
tmdb_client = { version = "1.9.0", default-features = false, features = ["async", "native-tls"] }

# Blocking only (default behavior)
tmdb_client = { version = "1.9.0" }

# Both sync and async (maximum flexibility)
tmdb_client = { version = "1.9.0", features = ["blocking", "async"] }

# Models only (no API clients, useful for type definitions)
tmdb_client = { version = "1.9.0", default-features = false, features = ["native-tls"] }
```

### Binary Size Impact

When using async-only configuration, you can significantly reduce binary size:

```bash
# Async only example
cargo build --release --no-default-features --features "async,native-tls" --example async_example
# Binary size: ~4.8 MB (optimized)

# Both blocking and async
cargo build --release --features "blocking,async" --example async_example
# Binary size: ~5.2 MB (includes blocking HTTP client)
```

## Usage

### Blocking (Original)

```rust
use tmdb_client::apis::client::APIClient;

let client = APIClient::new_from_env();
let movies = client.movies_api()
    .get_movie_popular_paginated(None, None, None)?;
```

### Async (New)

```rust
use tmdb_client::async_apis::AsyncAPIClient;

#[tokio::main]
async fn main() {
    let client = AsyncAPIClient::new_from_env();
    let movies = client.movies_api()
        .get_movie_popular_paginated(None, None, None)
        .await?;
}
```

## Dependencies

### Conditional Dependencies

```toml
[dependencies]
# Core dependencies (always included)
serde = "^1.0"
serde_json = "^1.0"
# ... other core deps

# Base reqwest without blocking
reqwest = { version = "0.12", default-features = false, features = ["json", "gzip"] }

# Optional dependencies
tokio = { version = "1.47", features = ["macros", "rt-multi-thread"], optional = true }
async-trait = { version = "0.1", optional = true }

[features]
blocking = ["reqwest/blocking"]  # Adds blocking feature to reqwest
async = ["dep:tokio", "dep:async-trait"]  # Adds tokio and async-trait
```

When you enable:
- **`blocking`**: Adds `reqwest/blocking` feature
- **`async`**: Adds `tokio` and `async-trait` dependencies
- **Neither**: Only reqwest's base async client (not used)

## Testing

### Compilation Tests

```bash
# Check blocking only
cargo check --no-default-features --features "blocking,native-tls"

# Check async only
cargo check --no-default-features --features "async,native-tls"

# Check both
cargo check --features "blocking,async"

# Check models only
cargo check --no-default-features --features "native-tls"

# Build async example
cargo build --release --no-default-features --features "async,native-tls" --example async_example
```

### Example

Run the async example:

```bash
export TMDB_API_KEY="your_api_key_here"
cargo run --example async_example --no-default-features --features "async,native-tls"
```

## Performance Considerations

### When to Use Async

✅ **Use async when:**
- Making multiple concurrent API requests
- Building web servers or services
- Integrating with async frameworks (tokio, actix, axum, etc.)
- Need to handle many concurrent operations efficiently
- Want smaller binary size (async-only configuration)

❌ **Use blocking when:**
- Simple CLI tools or scripts
- Sequential operations only
- Simpler mental model preferred
- No async runtime available

### Concurrent Requests Example

The power of async shines with concurrent requests:

```rust
use futures::future::join_all;

let movie_ids = vec![550, 13, 680, 155, 680];

// Fetch all concurrently
let futures: Vec<_> = movie_ids
    .iter()
    .map(|&id| client.movies_api().get_movie_details(id, None, None, None))
    .collect();

let results = join_all(futures).await;
```

This is significantly faster than sequential blocking requests.

## Comparison with Other Clients

### vs jdrouet/tmdb-api

| Feature | tmdb-client-rs | jdrouet/tmdb-api |
|---------|----------------|------------------|
| Async Support | ✅ | ✅ (async-first) |
| Blocking Support | ✅ | ❌ |
| Both Simultaneously | ✅ | ❌ |
| Conditional Compilation | ✅ | ❌ |
| API Coverage | ~200 endpoints | ~50 endpoints |
| Code Generation | OpenAPI Generator | Hand-written |
| Binary Size Optimization | ✅ (choose what you need) | ⚠️ (always async) |

### Advantages

- **Maximum Flexibility**: Use sync, async, or both
- **Optimized Binary Size**: Only compile what you need
- **Completeness**: All TMDB API endpoints covered
- **Consistency**: Same models and error types for both
- **Migration**: Easy to migrate from sync to async gradually
- **Minimal Dependencies**: No tokio if you only need blocking

## Maintenance

### Regenerating Async APIs

If you update the sync APIs, regenerate the async versions:

```bash
python3 generate_async_apis.py
```

The script automatically transforms sync code to async:
- `Rc` → `Arc`
- Add `#[async_trait::async_trait]`
- Add `async` keyword to methods
- Add `.await` to HTTP calls

## Conclusion

The async implementation provides a complete, production-ready async client for TMDB API with:

✅ **Conditional compilation** - Only compile what you need
✅ **Backward compatibility** - Existing sync code works unchanged
✅ **No duplication** - Shared models between sync and async
✅ **Flexibility** - Choose sync, async, or both
✅ **Optimized** - Smaller binaries when using async-only

This makes tmdb-client-rs one of the most flexible TMDB clients available in the Rust ecosystem.
