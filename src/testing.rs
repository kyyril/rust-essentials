// Topic 16: Testing
// Run with: cargo test
//          cargo test --bin testing
//          cargo test --doc

// We keep tests in the same file for demo purposes.
// In a real project each test lives in tests/<name>.rs.

// =========================================================================
// Code under test
// =========================================================================

/// Adds two positive integers; returns None for negative inputs.
pub fn add_pos(a: i32, b: i32) -> Option<i32> {
    if a < 0 || b < 0 {
        None
    } else {
        Some(a + b)
    }
}

/// Divides a by b; returns Err when b is zero.
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 { Err("division by zero") } else { Ok(a / b) }
}

/// Returns the n-th Fibonacci number (0-indexed).
pub fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

// =========================================================================
// 1. Unit tests — #[test] inside impl/bin
// =========================================================================
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn add_pos_happy_path() {
        assert_eq!(add_pos(2, 3), Some(5));
        assert_eq!(add_pos(0, 0), Some(0));
    }

    #[test]
    fn add_pos_negative_rejected() {
        assert_eq!(add_pos(-1, 2), None);
        assert_eq!(add_pos(2, -3), None);
    }

    #[test]
    fn divide_is_ok() {
        assert!((divide(10.0, 4.0).unwrap() - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn divide_by_zero_is_err() {
        assert_eq!(divide(1.0, 0.0), Err("division by zero"));
    }

    // --- should_panic — expect a panic ---
    #[test]
    #[should_panic]
    fn panic_example() {
        panic!("expected panic");
    }

    // --- should_panic with expected message ---
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn vector_oob_panics() {
        vec![1, 2, 3][99];
    }

    // --- Result-based tests (preferred over should_panic) ---
    #[test]
    fn result_bit() -> Result<(), String> {
        let r = add_pos(2, 2).ok_or("negative".to_string())?;
        assert_eq!(r, 4);
        Ok(())
    }
}

// =========================================================================
// 2. Integration tests — tests/<name>.rs (this file doubles as one)
// =========================================================================
/// These tests run against the public API only (pub functions).
#[test]
fn fib_sequence_is_correct() {
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(fib(i as u32), exp, "fib({})", i);
    }
}

// =========================================================================
// 3. Doc-tests — rustdoc extracts and runs them
// =========================================================================
/// Adds `a` and `b` when both are non-negative.
///
/// # Examples
///
/// ```
/// let result = rust_essentials::add_pos(1, 2);
/// assert_eq!(result, Some(3));
/// ```
///
/// Negative inputs return `None`:
///
/// ```
/// assert_eq!(rust_essentials::add_pos(-1, 1), None);
/// ```
pub fn doc_tested_add(a: i32, b: i32) -> Option<i32> {
    add_pos(a, b)
}

// =========================================================================
// 4. Test doubles — mock objects
// =========================================================================
/// Trait we want to mock in tests
trait Clock {
    fn now(&self) -> u64;
}

/// Production: real wall-clock
struct RealClock;
impl Clock for RealClock {
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Test: fake clock — controllable time
struct FakeClock(u64);
impl Clock for FakeClock {
    fn now(&self) -> u64 { self.0 }
}

/// greet_if_morning uses a Clock to decide what to say.
/// Clock is a trait object — inject FakeClock in tests.
struct Greeter<C: Clock> {
    clock: C,
}

impl<C: Clock> Greeter<C> {
    fn new(clock: C) -> Self { Self { clock } }

    fn greet(&self) -> String {
        let h = (self.clock.now() / 3600) % 24;
        if (6..12).contains(&h) {
            format!("Good morning! ({h}:00)")
        } else {
            "Hello!".to_string()
        }
    }
}

// =========================================================================
// 5. Property-based tests (proptest — conceptual)
// =========================================================================
// Feature-gated so this file compiles without the dependency:
#[cfg(feature = "proptest")]
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn fib_is_never_negative(n: u32) {
            assert!(fib(n) >= 0);
        }

        #[test]
        fn add_pos_result_matches_ref(a: u32, b: u32) {
            prop_assert_eq!(add_pos(a as i32, b as i32), Some(a as i32 + b as i32));
        }
    }
}

// =========================================================================
// 6. Benchmarks (cargo bench)
// =========================================================================
#[cfg(test)]
mod bench_demo {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_fib_30(b: &mut Bencher) {
        b.iter(|| fib(black_box(30)));
    }
}

// =========================================================================
// 7. main() — run integration tests manually
// =========================================================================
fn main() {
    // Unit tests are run by `cargo test`; this is for manual smoke-test.
    assert_eq!(add_pos(1, 2), Some(3));
    assert_eq!(add_pos(-1, 2), None);
    assert!((divide(10.0, 4.0).unwrap() - 2.5).abs() < f64::EPSILON);
    assert_eq!(fib(9), 34);

    println!("Integration smoke-tests passed.");
}