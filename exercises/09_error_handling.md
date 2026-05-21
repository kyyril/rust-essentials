# Exercise 9 — Error Handling

## Task 9.1 — Option in practice

```rust
fn second_even(nums: &[i32]) -> Option<i32> {
    // TODO: return the second even number in `nums`, or None if none found.
    //   nums = [1, 4, 7, 8]  → Some(8)
    //   nums = [1, 3, 5]     → None
    //   nums = [2, 4, 6, 8]  → Some(4)
    // Write it with .filter() and .nth() or a simple for.
}

fn main() {
    println!("{:?}", second_even(&[1, 4, 7, 8]));
    println!("{:?}", second_even(&[1, 3, 5]));
    println!("{:?}", second_even(&[2, 4, 6, 8]));
}
```

---

## Task 9.2 — Propagate errors with `?`

```rust
use std::fs;

fn main() {
    // TODO: call read_to_string with "data.txt" and propagate with ?
    //       then print the first 80 characters.
}
```

---

## Task 9.3 — Early return on error

```rust
fn parse_positive(s: &str) -> Result<i32, &str> {
    // TODO:
    //   1. trim leading/trailing whitespace
    //   2. parse to i32
    //   3. if parsing fails → Err("not a number")
    //   4. if parsed value ≤ 0 → Err("must be positive")
    //   5. otherwise Ok(value)
}
```

---

## Task 9.4 — Custom error type with `thiserror`-style

If you have the `thiserror` crate available, write:

```rust
// TODO: define a `AppError` enum with variants:
//   FileNotFound(path: String)
//   ParseError(msg: String)
//   NegativeNumber(n: i32)
// Derive Error, Display, and Debug using thiserror attributes.
```

---

## Task 9.5 — when to use which

Fill in the table:

| Task | Use |
|---|---|
| User input that may be absent | `Option` |
| Reading a file that may not exist | `Result` |
| Division by zero (logic bug) | `panic!` or return `Option` |
| Validate user-entered number | `Result` |

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 9.1
fn second_even(nums: &[i32]) -> Option<i32> {
    let mut evens = nums.iter().filter(|&&n| n % 2 == 0);
    evens.next();                   // skip first
    evens.next()                    // take second (None if insufficient)
}

// 9.2
fn main() {
    let content = fs::read_to_string("data.txt")?; // prop.
    println!("{}", &content[..80.min(content.len())]);
}

// 9.3
fn parse_positive(s: &str) -> Result<i32, &str> {
    let n = s.trim().parse::<i32>().map_err(|_| "not a number")?;
    if n <= 0 { return Err("must be positive"); }
    Ok(n)
}

// 9.4
#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("File not found: {0}")]   FileNotFound(String),
    #[error("Parse error: {0}")]      ParseError(String),
    #[error("Negative number: {0}")]  NegativeNumber(i32),
}
```
</details>
