# Exercise 4 — Control Flow

> Try it on your own first.

## Task 4.1 — FizzBuzz with match

Write a function `fizz_buzz(n: u32)` that returns:

| n | Return |
|---|---|
| divisible by 3 & 5 | `"FizzBuzz"` |
| divisible by 3 | `"Fizz"` |
| divisible by 5 | `"Buzz"` |
| otherwise | the number as a string |

Use a **`match`** expression.

---

## Task 4.2 — Break with a labelled loop

```rust
fn find_pair() -> (u32, u32) {
    // Find the FIRST pair (a, b) where a and b are in the range 1..=20
    // and a * b > 100. Return (a, b) using a labelled loop + break value.
}
```

---

## Task 4.3 — Collect odds into a Vec

```rust
fn collect_odds(n: u32) -> Vec<u32> {
    // Collect all odd numbers from 1 to n (inclusive) into a Vec.
}
```

---

## Task 4.4 — if let unwrap

```rust
fn get_score(op: Option<i32>) -> i32 {
    // TODO: return the value inside Some, or -1 if it is None.
    // Use `if let` — not match.
}
```

---

## Task 4.5 — let else pattern

```rust
fn sqrt_int(v: Option<i32>) -> i32 {
    // TODO: return the integer's square root as i32 if Some(v) and v >= 0.
    // For None or v < 0, return 0. Use `let ... else`.
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 4.1
fn fizz_buzz(n: u32) -> &'static str {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz",
        (0, _) => "Fizz",
        (_, 0) => "Buzz",
        _ => "other",
    }
}

// 4.2
fn find_pair() -> (u32, u32) {
    'outer: for a in 1..=20 {
        for b in 1..=20 {
            if a * b > 100 {
                break 'outer (a, b);
            }
        }
    }
    (0, 0) // fallback (never reached)
}

// 4.3
fn collect_odds(n: u32) -> Vec<u32> {
    let mut odds = Vec::new();
    for i in 1..=n {
        if i % 2 == 1 {
            odds.push(i);
        }
    }
    odds
}

// 4.4
fn get_score(op: Option<i32>) -> i32 {
    if let Some(v) = op { v } else { -1 }
}

// 4.5
fn sqrt_int(v: Option<i32>) -> i32 {
    let Some(v) = v else { return 0; };
    if v < 0 { 0 } else { (v as f64).sqrt() as i32 }
}
```
</details>
