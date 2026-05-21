# Exercise 16 — Testing

All exercises use `cargo test` as the orchestrator where noted.

---

## Task 16.1 — assert_eq! vs assert_ne!

```rust
// TODO: complete the assertions below
#[test]
fn assert_works() {
    assertEq!();  // 2 + 2 == 4
    assertNe!();  // "cat" != "dog"
    // assert!(2 + 2 == 5, "impossible"); // uncomment to see a nice diff
}
```

---

## Task 16.2 — Result-based test

```rust
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse()
}

#[test]
fn parse_valid() -> Result<(), &str> {
    let n = parse_int(" 42 ")?; // TODO: propagate or check
    assert_eq!(n, 42);
    Ok(())
}

#[test]
fn parse_invalid() -> Result<(), &str> {
    // TODO: verify parse_int("abc") returns Err
    Ok(())
}
```

---

## Task 16.3 — Mock via trait object

```rust
trait Clock { fn now(&self) -> u64; }

// TODO: expose a FakeClock(u64) that always returns the stored value.
struct Greeter<C: Clock> { clock: C }
impl<C: Clock> Greeter<C> {
    fn hour(&self) -> u8 { ((self.clock.now() / 3600) % 24) as u8 }
}
fn is_morning(p: &impl Clock) -> bool {
    (6..12).contains(&(p.now() / 3600 % 24) as u8)
}

#[test]
fn morning_is_true_at_09() {
    // TODO: 09:00 UTC = 9 * 3600 seconds
}
```

---

## Task 16.4 — Doc-test

```rust
/// Returns `a + b` for any `i32` pair.
///
/// # Examples
///
/// ```
/// assert_eq!(rust_essentials::add(-1, 1), 0);  // TODO: fix import path
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

Fix the import path in the example and run `cargo test --doc`.

---

## Task 16.5 — Integration test

Create **`tests/add_test.rs`**:
```rust
use rust_essentials::*;
#[test] fn add_test() { assert_eq!(add(3, 4), 7); }
```

Run: `cargo test --test add_test`

---

## Task 16.6 — Property test with `proptest`

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1"
```

```rust
proptest! {
    #[test]
    fn abs_is_non_negative(n: i32) {
        prop_assert!(n.abs() >= 0);
        prop_assert_eq!(n.abs(), (-n).abs()); // symmetry
    }
}
```

Run: `cargo test --test proptest_abs`

---

## Task 16.7 — Benchmark `fib`

```rust
#![feature(test)]
extern crate test;
use test::{Bencher, black_box};

fn fib(n: u32) -> u64 { /* iter */ }

#[bench]
fn bench_fib(b: &mut Bencher) {
    b.iter(|| fib(black_box(30)));
}
```

Run: `cargo bench`

---

## Answers

<details>
<summary>Click to reveal</summary>

```rust
// 16.1
assert_eq!(2 + 2, 4);
assert_ne!("cat", "dog");

// 16.2
let n = parse_int(" 42 ")?;
assert!(parse_int("abc").is_err());

// 16.3
struct FakeClock(u64);
impl Clock for FakeClock { fn now(&self) -> u64 { self.0 } }
#[test]
fn morning_at_9() {
    assert!(is_morning(&FakeClock(9 * 3600)));
}

// 16.4 — path in crate root:  rust_essentials::add

// 16.5 — file at tests/add_test.rs:
use rust_essentials::add;
#[test] fn add_test() { assert_eq!(add(3,4), 7); }

// 16.6 — proptest! macro from proptest crate

// 16.7 — bench using test crate Bencher + black_box
```
</details>
