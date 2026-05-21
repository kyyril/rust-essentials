# 9 — Error Handling

## Overview

Rust does not have exceptions. Instead, two standard library types communicate errors at the type level, and the compiler checks that every error case is handled.

---

## Two Kinds of Failure

| Type | Represents | When to use |
|---|---|---|
| `Option<T>` | Value may be absent `None`/`Some(T)` | Optional, non-error absence |
| `Result<T, E>` | Operation may fail with reason `Err(E)` | Expected failure with diagnostics |

---

## Recoverable errors without Result/Option — panic!

`panic!` stops the program immediately (like an assertion).

```rust
panic!("this is unrecoverable");
```

When it happens, Rust prints a **panic message + backtrace** (enable with `RUST_BACKTRACE=1 cargo run`).

---

## Result<T, E>

The `Result` enum is defined as:

```rust
enum Result<T, E> {
    Ok(T),  // success, holds the value
    Err(E), // failure, holds an error
}
```

### Explicit windowing with match

```rust
use std::fs::File;
let f = match File::open("hello.txt") {
    Ok(file)  => file,
    Err(error) => {
        panic!("Cannot open file: {error}");
    }
};
```

### The ? operator

Propagate errors upward with a single character — the calling function receives the error:

```rust
fn read_file() -> io::Result<String> {
    let mut f = File::open("hello.txt")?; // short for match
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` converts between error types using `From`, so returning a different error enum works seamlessly.

---

## Option<T>

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
```

### Option combinators

```rust
let v = Some("42")
    .and_then(|s| s.parse::<i32>().ok())
    .map(|n| n * 2);
println!("{:?}", v); // Some(84)
```

---

## Result combinators

```rust
let lines = fs::read_to_string("file.txt")
    .map(|s| s.lines().collect())
    .and_then(|v: Vec<&str>| Ok(v.len()));
```

| Method | Effect |
|---|---|
| `.map(|v|…)` | transform Ok |
| `.map_err(|e|…)` | transform Err |
| `.and_then(|v|…)` | flatMap for Ok |
| `.or_else(|e|…)` | recover from Err |
| `.unwrap_or(v)` | substitute on Err |
| `.unwrap_or_else(f)` | compute substitute |

---

## when to use which

| Situation | Use |
|---|---|
| Missing optional value | `Option` |
| IO, parsing, network error | `Result` |
| Programmer bug / contract violated | `panic!` |
| Prototyping / unwrap on已知-safe value | `.unwrap()` / `.expect()` |

---

## Custom error types

Implement `std::error::Error` for your domain errors to enable `?` propagation:

```rust
#[derive(Debug)]
enum MyErr { Io(std::io::Error), Parse(std::num::ParseIntError) }
impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr { … }
```

Then use `thiserror` crate in production for less boilerplate.
