# CLAUDE.MD - Rust Project Configuration

## Project Overview
**Project Type**: Rust Application
**Primary Goal**: Memory-safe, performant systems programming
**Core Technology**: Rust with Cargo build system

---

## CRITICAL EXECUTION RULES

### Golden Rule: 1 MESSAGE = ALL MEMORY-SAFE OPERATIONS

**ALL Rust operations MUST be concurrent/parallel in a single message:**

1. **Cargo Operations** - ALWAYS batch ALL cargo build/test/run commands together
2. **Crate Management** - ALWAYS batch ALL dependency installations together
3. **Testing** - ALWAYS run ALL test suites in parallel via `cargo test`
4. **Memory Safety** - ALWAYS batch ALL borrowing/ownership patterns together
5. **Concurrency** - ALWAYS batch ALL async/threading implementations together

---

## Agent Roles & Specializations

### Systems Architect
- **Focus**: Memory management, ownership patterns, lifetime annotations
- **Tools**: rustc, cargo, rust-analyzer
- **Responsibilities**: Design safe system interfaces, manage resource lifetimes

### Performance Engineer
- **Focus**: Zero-cost abstractions, SIMD, profiling
- **Tools**: criterion, flamegraph, perf, cargo-benchcmp
- **Responsibilities**: Optimize hot paths, eliminate allocations, parallel processing

### Safety Specialist
- **Focus**: Borrow checker compliance, lifetime management, unsafe code review
- **Tools**: miri, cargo-geiger, rust-clippy
- **Responsibilities**: Ensure memory safety, validate unsafe blocks

### Concurrency Expert
- **Focus**: Async/await, tokio runtime, channels, parallelism
- **Tools**: tokio, rayon, crossbeam, async-std
- **Responsibilities**: Design concurrent systems, prevent data races

### Testing Agent
- **Focus**: Unit tests, integration tests, property testing, fuzzing
- **Tools**: cargo test, proptest, cargo-fuzz, tarpaulin
- **Responsibilities**: Comprehensive test coverage, edge case discovery

### Ecosystem Agent
- **Focus**: Crate selection, FFI, WebAssembly, platform compatibility
- **Tools**: crates.io, cargo-audit, wasm-pack
- **Responsibilities**: Dependency management, cross-compilation, security audits

---

## Build & Project Coordination

### Cargo Configuration
```toml
[package]
name = "better_brew"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"

[dependencies]
# Core dependencies
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"

[dev-dependencies]
proptest = "1.0"
criterion = "0.5"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[[bench]]
name = "benchmarks"
harness = false
```

### Concurrent Build Commands
```bash
# ALWAYS batch these together in a single message:
cargo build --release && \
cargo test --all-features && \
cargo clippy -- -D warnings && \
cargo fmt --check
```

---

## Testing Strategy

### Comprehensive Test Batching
```rust
// Unit Tests - ALWAYS batch with integration tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_transfer() {
        let data = vec![1, 2, 3];
        let result = process_data(data);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = async_fetch().await;
        assert!(result.is_ok());
    }
}

// Property Tests
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_reversible(input in prop::collection::vec(any::<u32>(), 0..100)) {
            let reversed = reverse_twice(input.clone());
            prop_assert_eq!(input, reversed);
        }
    }
}
```

### Test Execution Pattern
```bash
# Run ALL tests in parallel:
cargo test --all-features --all-targets && \
cargo test --doc && \
cargo bench --no-run
```

---

## Memory Safety Patterns

### Ownership & Borrowing
```rust
// ALWAYS batch related ownership patterns:

// 1. Ownership Transfer
fn take_ownership(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}

// 2. Immutable Borrowing
fn read_data(data: &Vec<i32>) -> i32 {
    data.iter().sum()
}

// 3. Mutable Borrowing
fn modify_data(data: &mut Vec<i32>) {
    data.iter_mut().for_each(|x| *x *= 2);
}

// 4. Lifetime Management
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## Concurrency & Async Patterns

### Tokio Runtime Configuration
```rust
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

// ALWAYS batch async setup:
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Channel Communication
    let (tx, mut rx) = mpsc::channel(100);

    // 2. Shared State
    let shared_data = Arc::new(Mutex::new(Vec::new()));

    // 3. Spawn Tasks
    let handle1 = tokio::spawn(async move {
        // Task 1 logic
    });

    let handle2 = tokio::spawn(async move {
        // Task 2 logic
    });

    // 4. Join All
    tokio::try_join!(handle1, handle2)?;

    Ok(())
}
```

### Rayon Parallelism
```rust
use rayon::prelude::*;

fn parallel_processing(data: Vec<i32>) -> Vec<i32> {
    data.par_iter()
        .map(|&x| expensive_computation(x))
        .collect()
}
```

---

## Web Development Patterns

### Axum Server Setup
```rust
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/data", post(data_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

### Database Integration (SQLx)
```rust
use sqlx::{PgPool, postgres::PgPoolOptions};

async fn setup_database() -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://localhost/mydb")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
```

---

## Performance Optimization

### Zero-Copy Patterns
```rust
use std::borrow::Cow;

fn process_string(input: &str) -> Cow<str> {
    if input.contains("special") {
        Cow::Owned(input.replace("special", "normal"))
    } else {
        Cow::Borrowed(input)
    }
}
```

### SIMD Optimization
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn simd_sum(data: &[f32]) -> f32 {
    // SIMD implementation for performance-critical paths
    data.iter().sum()
}
```

### Benchmarking
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

---

## Security Practices

### Input Validation
```rust
use validator::Validate;

#[derive(Debug, Validate)]
struct UserInput {
    #[validate(length(min = 1, max = 100))]
    username: String,

    #[validate(email)]
    email: String,
}

fn validate_input(input: UserInput) -> Result<UserInput, validator::ValidationErrors> {
    input.validate()?;
    Ok(input)
}
```

### Cryptographic Operations
```rust
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;

fn hash_password(password: &[u8]) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password, &salt)?
        .to_string();
    Ok(hash)
}
```

### Dependency Auditing
```bash
# ALWAYS batch security checks:
cargo audit && \
cargo deny check && \
cargo outdated
```

---

## Deployment Configuration

### Dockerfile
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/better_brew /usr/local/bin/
EXPOSE 8080
CMD ["better_brew"]
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: better-brew
spec:
  replicas: 3
  selector:
    matchLabels:
      app: better-brew
  template:
    metadata:
      labels:
        app: better-brew
    spec:
      containers:
      - name: better-brew
        image: better-brew:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "64Mi"
            cpu: "250m"
          limits:
            memory: "128Mi"
            cpu: "500m"
```

### Systemd Service
```ini
[Unit]
Description=Better Brew Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/better_brew
ExecStart=/usr/local/bin/better_brew
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

---

## Code Quality Standards

### Rustfmt Configuration (.rustfmt.toml)
```toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Auto"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
```

### Clippy Configuration
```bash
# Run with strict lints:
cargo clippy -- \
  -D warnings \
  -D clippy::all \
  -D clippy::pedantic \
  -A clippy::missing_errors_doc
```

### Documentation
```rust
/// Processes input data and returns transformed result.
///
/// # Arguments
///
/// * `data` - The input vector to process
///
/// # Returns
///
/// A new vector with transformed values
///
/// # Examples
///
/// ```
/// let input = vec![1, 2, 3];
/// let result = process_data(input);
/// assert_eq!(result, vec![2, 4, 6]);
/// ```
pub fn process_data(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}
```

---

## CI/CD Pipeline

### GitHub Actions Workflow (.github/workflows/ci.yml)
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Build
        run: cargo build --verbose --all-features

      - name: Test
        run: cargo test --verbose --all-features

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Format Check
        run: cargo fmt -- --check

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Security Audit
        run: cargo audit

      - name: Dependency Check
        run: cargo deny check

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate Coverage
        run: cargo tarpaulin --out Xml

      - name: Upload Coverage
        uses: codecov/codecov-action@v3
```

---

## Best Practices Summary

### Memory Safety
- ✅ Understand ownership, borrowing, and lifetimes deeply
- ✅ Minimize unsafe code; audit all unsafe blocks thoroughly
- ✅ Use smart pointers (Box, Rc, Arc) appropriately
- ✅ Leverage the borrow checker as a design tool

### Performance
- ✅ Profile before optimizing (perf, flamegraph)
- ✅ Use zero-cost abstractions (iterators, generics)
- ✅ Avoid unnecessary allocations and clones
- ✅ Parallelize with rayon for CPU-bound tasks
- ✅ Use async/await for I/O-bound operations

### Error Handling
- ✅ Use Result<T, E> for recoverable errors
- ✅ Use Option<T> for optional values
- ✅ Leverage the ? operator for error propagation
- ✅ Provide context with anyhow or thiserror

### Concurrency
- ✅ Prefer message passing (channels) over shared state
- ✅ Use Arc<Mutex<T>> for shared mutable state
- ✅ Understand Send and Sync traits
- ✅ Avoid blocking the async runtime

### Testing
- ✅ Write unit tests alongside code
- ✅ Use integration tests for public APIs
- ✅ Employ property-based testing for complex logic
- ✅ Fuzz test parsers and input handlers
- ✅ Benchmark performance-critical code

### Code Quality
- ✅ Run rustfmt on all code
- ✅ Address all clippy warnings
- ✅ Document public APIs with examples
- ✅ Keep dependencies minimal and audited
- ✅ Use semantic versioning

---

## Essential Tools

- **rustc** - The Rust compiler
- **cargo** - Build system and package manager
- **rust-analyzer** - LSP implementation for IDE support
- **clippy** - Linting tool for common mistakes
- **rustfmt** - Code formatter
- **cargo-audit** - Security vulnerability scanner
- **cargo-deny** - Dependency policy enforcement
- **criterion** - Benchmarking framework
- **proptest** - Property-based testing
- **tarpaulin** - Code coverage tool
- **flamegraph** - Profiling visualization
- **miri** - Interpreter for detecting undefined behavior

---

## Getting Started Checklist

- [ ] Initialize project with `cargo init` or `cargo new`
- [ ] Configure Cargo.toml with appropriate dependencies
- [ ] Set up .rustfmt.toml and .clippy.toml
- [ ] Create CI/CD pipeline (GitHub Actions)
- [ ] Write initial tests (unit + integration)
- [ ] Configure release profile for optimization
- [ ] Set up pre-commit hooks for formatting and linting
- [ ] Document public APIs
- [ ] Run security audit on dependencies
- [ ] Create Dockerfile for containerized deployment

---

## Project-Specific Notes

*Add project-specific configuration, architecture decisions, and team conventions here.*

---

**Remember**: ALWAYS batch operations together. NEVER execute cargo commands, tests, or async operations sequentially when they can run in parallel within a single message.
