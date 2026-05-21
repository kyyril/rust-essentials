# Exercise 3 — Functions

> Complete each task before viewing the solution.

## Task 3.1 — Returning from a function

```rust
// TODO: swap returns so it returns (b, a)
fn swap(a: i32, b: i32) -> i32 {
    a
}
```

---

## Task 3.2 — Expression body

```rust
// TODO: rewrite the body as a single expression (no braces, no return keyword)
fn square(n: i32) -> i32 {
    let result = n * n;
    return result;
}
```

---

## Task 3.3 — Multiple return values via tuple

Write a function `stats` that takes a slice of `&[i32]` and returns the **minimum**
and **maximum** as a tuple `(min, max)`.

```rust
fn stats(nums: &[i32]) -> (i32, i32) {
    // ── TODO ──
    // If the slice is empty, return (0, 0).
    // Otherwise iterate, tracking min and max.
}
```

---

## Task 3.4 — Function order

```rust
// What happens if you call main() from println! above its definition?
// Try it and fix it in one of two ways:
//   a) Move main() after the helper function definition
//      — this is the idiomatic fix ✅
//   b) Add a forward declaration (not idiomatic in Rust, but possible)
```

---

## Task 3.5 — Consuming vs borrowing

```rust
struct Color(i32, i32, i32);

fn to_string_owned(c: Color) -> String {
    format!("rgb({}, {}, {})", c.0, c.1, c.2)
}

fn main() {
    let red = Color(255, 0, 0);
    println!("{}", to_string_owned(red));
    // TODO: fix to_string_owned and its call so `red`
    //       is borrowed (&) but still works correctly.
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 3.1
fn swap(a: i32, b: i32) -> i32 {
    b   // simply return b
}

// 3.2 — expression body
fn square(n: i32) -> i32 {
    n * n
}

// 3.3
fn stats(nums: &[i32]) -> (i32, i32) {
    if nums.is_empty() { return (0, 0); }
    let mut min = nums[0];
    let mut max = nums[0];
    for &n in &nums[1..] {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

// 3.4 — Move main() below helper fn or use the idiomatic fix.
// No forward declarations in Rust: organise top-down or use modules.

// 3.5
fn to_string_owned(c: &Color) -> String {    // borrow instead of move
    format!("rgb({}, {}, {})", c.0, c.1, c.2)
}
fn main() {
    let red = Color(255, 0, 0);
    println!("{}", to_string_owned(&red));   // pass reference
}
```
</details>
