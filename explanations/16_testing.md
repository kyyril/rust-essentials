# 16 — Testing

## Overview

Rust has built-in support for unit tests, integration tests, doc-tests, benchmarks, and property tests. `cargo test` discovers and runs everything automatically.

---

## 1. Unit tests — `#[test]` in the same file

Unit tests are written **inside the file they test** — in a `#[cfg(test)]` module so they are not compiled in release builds.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_is_correct() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_panics() {
        div(10, 0);
    }
}
```

### Built-in assertion macros

| Macro | Fails when |
|---|---|
| `assert!(cond)` | `cond == false` |
| `assert_eq!(a, b)` | `a != b` (formats diff) |
| `assert_ne!(a, b)` | `a == b` |
| `debug_assert!(cond)` | Only in debug builds |
| `matches!(val, pat)` | `val !~ pat` |
| `assert_matches!(val, pat)` | Hard-fail version |

### Result-based tests

Return `Result<(), E>` for early exit — **preferred over `#[should_panic]`**:

```rust
#[test]
fn parse_ok() -> Result<(), MyErr> {
    let v = parse("42")?;
    assert_eq!(v, 42);
    Ok(())
}
```

### Should-panic variants

```rust
#[should_panic]                    // any panic
#[should_panic(expected = "msg")]  // panic message must contain "msg"
```

---

## 2. Integration tests — `tests/<name>.rs`

Run against the **public API only**. Each file in `tests/` is a separate crate:

```
my_crate/
├── src/
│   └── lib.rs
└── tests/
    ├── addition_test.rs   ← separate crate, imports my_crate
    └── parse_test.rs
```

```rust
// tests/addition_test.rs
use my_crate::*;

#[test]
fn integration_add() {
    assert_eq!(add(1, 1), 2);
}
```

Run: `cargo test --test addition_test`

Run all integration tests: `cargo test`

---

## 3. Doc-tests — `cargo test --doc`

Rustdoc extracts ` ```rust ` **code blocks** from doc comments and runs them:

```rust
/// Adds `a` and `b`.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run: `cargo test --doc`

Use `no_compile` / `ignore` directives for examples that can't run in CI:

```rust
/// ```no_run
/// // won't be executed by doctest
/// startup_server();
/// ```
```

---

## 4. Test doubles — trait objects as mocks

Traits naturally support **duck-typed substitution** without a mocking framework:

```rust
trait Clock { fn now(&self) -> u64; }

struct RealClock;   // production
struct Fake(u64);   // test

impl Clock for RealClock { fn now(&self) -> u64 { /* real time */ } }
impl Clock for Fake    { fn now(&self) -> u64 { self.0 } }

// Greeter<C: Clock> works with both automatically
let g = Greeter::new(Fake(9 * 3600));  // 09:00 UTC → "Good morning"
```

More elaborate: use `mockall` crate for `#[automock]` generated mocks, or `double` / `fake` for simpler cases.

---

## 5. Property-based tests — `proptest`

Property tests generate **hundreds of random inputs** and assert invariants hold for all of them:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn add_is_commutative(a: i32, b: i32) {
        prop_assert_eq!(add(a, b), add(b, a));
    }
}
```

Cargo.toml:
```toml
[dev-dependencies]
proptest = "1"
```

`#[should_panic]` with `proptest` also works — the test only passes if the panic is reproducible across random inputs.

---

## 6. Benchmarks — `cargo bench`

Benchmarks live in ` benches/<name>.rs `, use the `test` crate:

```rust
#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn bench_fib_30(b: &mut Bencher) {
    b.iter(|| fib(black_box(30)));
}

// black_box prevents the optimizer from computing the result at compile time
```

Run: `cargo bench`

---

## 7. Test isolation

Each test runs in a fresh scope. Use `tempfile` crate instead of writing to `/tmp` manually:
```rust
use tempfile::NamedTempFile;
let tmp = NamedTempFile::new().expect("create temp");
```

---

## 8. CI / GitHub Actions snippet

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with: { toolchain: stable, override: true }
      - run: cargo test --all-features --verbose
      - run: cargo bench
```

---

## Quick reference

| Command | What it runs |
|---|---|
| `cargo test` | Everything (unit + integration + doc) |
| `cargo test --lib` | Unit tests only |
| `cargo test --tests` | Integration tests only |
| `cargo test --doc` | Doc-tests only |
| `cargo test some_name` | Tests whose name contains `some_name` |
| `cargo test -- --nocapture` | Show `println!` output |
| `cargo bench` | All benchmarks |
