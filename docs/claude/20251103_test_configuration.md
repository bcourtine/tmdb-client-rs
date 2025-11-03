# Test Configuration Guide

## Overview

The test suite has been configured to work with the conditional compilation system. Tests are only compiled and run when their required features are enabled.

## Test Categories

### 1. Blocking API Tests (require `blocking` feature)

**Files:**
- `alternative_titles_api_tests.rs`
- `collections_api_tests.rs`
- `companies_api_tests.rs`
- `configuration_api_tests.rs`
- `movie_api_tests.rs`
- `networks_api_tests.rs`
- `people_api_tests.rs`
- `search_api_tests.rs`
- `trending_api_tests.rs`
- `tv_episodes_api_tests.rs`

**Configuration:**
```rust
#![cfg(feature = "blocking")]
```

These tests use `tmdb_client::apis::client::APIClient` and require the `blocking` feature to compile.

### 2. Daily Exports Tests (require `daily-exports` feature)

**Files:**
- `external_id_tests.rs`

**Configuration:**
```rust
#![cfg(feature = "daily-exports")]
```

These tests use `tmdb_client::files::exports` and require the `daily-exports` feature to compile.

## Running Tests

### Default (all blocking tests, no daily-exports)

```bash
export TMDB_API_KEY="your_api_key_here"
cargo test
```

**Result:** Runs all API tests except daily-exports tests.

### With Daily Exports

```bash
export TMDB_API_KEY="your_api_key_here"
cargo test --features "daily-exports"
```

**Result:** Runs all API tests including daily-exports tests.

### Without Any Tests

```bash
cargo test --no-default-features --features "native-tls"
```

**Result:**
- Compiles test files but they contain 0 tests
- All tests are behind `#[cfg]` gates
- Useful for verifying models compile correctly

### Async Only (no tests available yet)

```bash
cargo test --no-default-features --features "async,native-tls"
```

**Result:** No tests run (async tests not yet implemented).

## Test Behavior by Feature

| Features | API Tests | Export Tests | Total Tests |
|----------|-----------|--------------|-------------|
| `blocking` only | ✅ Run | ❌ Skip | ~10 tests |
| `daily-exports` only | ❌ Skip | ⚠️ Won't compile | 0 (needs blocking too) |
| `blocking,daily-exports` | ✅ Run | ✅ Run | ~17 tests |
| `async` only | ❌ Skip | ❌ Skip | 0 |
| None | ❌ Skip | ❌ Skip | 0 |

**Note:** Daily exports tests also need `blocking` because they use the synchronous API client to verify the parsed IDs.

## Test File Structure

### Blocking API Test Template

```rust
#![cfg(feature = "blocking")]

use tmdb_client::apis::client::APIClient;

#[test]
fn some_api_test() {
    let client = APIClient::new_from_env();
    // Test API calls
}
```

### Daily Exports Test Template

```rust
#![cfg(feature = "daily-exports")]

use chrono::{Duration, Local};
use tmdb_client::files::exports;

#[test]
fn some_export_test() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_movie_ids(date.naive_local().date());
    assert!(exported_ids.expect("Download error").len() > 0);
}
```

## Continuous Integration

### Recommended CI Test Matrix

```yaml
# .github/workflows/test.yml
jobs:
  test:
    strategy:
      matrix:
        features:
          - "blocking"                    # Standard API tests
          - "blocking,daily-exports"      # All tests
          - "async"                       # Future async tests
          - ""                            # Models only (no tests)
    steps:
      - name: Run tests
        run: |
          if [ -z "${{ matrix.features }}" ]; then
            cargo test --no-default-features --features "native-tls"
          else
            cargo test --no-default-features --features "${{ matrix.features }},native-tls"
          fi
        env:
          TMDB_API_KEY: ${{ secrets.TMDB_API_KEY }}
```

## Adding New Tests

### For Blocking API Tests

1. Create test file in `tests/` directory
2. Add `#![cfg(feature = "blocking")]` at the top
3. Import `tmdb_client::apis::client::APIClient`
4. Write tests using the blocking client

### For Daily Exports Tests

1. Add test to `tests/external_id_tests.rs` (or create new file)
2. Add `#![cfg(feature = "daily-exports")]` at the top
3. Import `tmdb_client::files::exports`
4. Write tests using the exports functions

### For Future Async Tests

1. Create test file in `tests/async/` directory (to be created)
2. Add `#![cfg(feature = "async")]` at the top
3. Import `tmdb_client::async_apis::AsyncAPIClient`
4. Use `#[tokio::test]` attribute for async tests

Example:
```rust
#![cfg(feature = "async")]

use tmdb_client::async_apis::AsyncAPIClient;

#[tokio::test]
async fn async_movie_test() {
    let client = AsyncAPIClient::new_from_env();
    let result = client.movies_api()
        .get_movie_details(19995, None, None, None)
        .await;
    assert!(result.is_ok());
}
```

## Troubleshooting

### Tests don't compile

**Problem:** `cargo test` fails with compilation errors about missing modules.

**Solution:** Ensure you have the required features enabled:
```bash
cargo test --features "blocking"
```

### Tests compile but don't run (0 tests)

**Problem:** `cargo test` shows `test result: ok. 0 passed; 0 failed`.

**Cause:** The required feature is not enabled, so all tests are behind `#[cfg]` gates.

**Solution:** Enable the appropriate feature:
```bash
# For API tests
cargo test --features "blocking"

# For export tests
cargo test --features "blocking,daily-exports"
```

### External ID tests fail

**Problem:** Tests compile and run but fail with download or parsing errors.

**Possible causes:**
1. Network issues
2. TMDB daily export files not available for yesterday's date
3. File format changed

**Solution:** Check the date and try a different date, or skip these tests:
```bash
cargo test --features "blocking" -- --skip external_id
```

## Summary

The test suite is now fully integrated with the conditional compilation system:

✅ **Tests respect feature flags** - Only compile when needed
✅ **No test failures on minimal builds** - 0 tests is valid
✅ **Clear separation** - API tests vs export tests
✅ **Future-ready** - Structure supports async tests
✅ **CI-friendly** - Can test different feature combinations

This ensures that users can build with minimal features without encountering test compilation errors.
