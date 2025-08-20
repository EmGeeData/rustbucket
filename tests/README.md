# Tests folder

RustBucket uses Rust's built-in testing framework.

## Run all tests
```bash
cargo test
```

## Run specific test suites
```bash
# Unit tests only
cargo test --lib

# Integration tests only  
cargo test --test integration_test

# Benchmarks
cargo bench
```

## Test Coverage
- **Unit Tests**: 38 tests covering individual components
- **Integration Tests**: 5 tests covering full pipeline functionality
- **Total Coverage**: CLI parsing, palette loading, image processing, benchmarking
