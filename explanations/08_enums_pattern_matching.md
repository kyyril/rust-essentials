# 8 — Enums and Pattern Matching

## Overview

An enum is a type that can be **one of several possible variants**. Each variant can optionally carry associated data.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

An enum variable always holds **exactly one variant** — it is impossible to have both V4 and V6 at the same time.

---

## Enum Variants

| Kind | Syntax | Use |
|---|---|---|
| C-like | `enum` variants with no data | States, tokens |
| Struct variant | `Move { x: i32, y: i32 }` | Named fields inside variant |
| Tuple variant | `Write(String)` | Value attached to variant |

---

## Pattern Matching with `match`

`match` is the primary tool for working with enums. It **must be exhaustive** — every possible value must be handled:

```rust
fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

If you forget a variant, the compiler errors.

### The wildcard `_`

Use `_` when you don't care about remaining values:

```rust
match some_value {
    1 => "one",
    2 => "two",
    _ => "anything else",
}
```

---

## Binding data in match arms

```rust
enum Message {
    Move { x: i32, y: i32 },
    Write(String),
}

match msg {
    Message::Write(text) => println!("Message: {text}"),
    Message::Move { x, y } => println!("Move to ({x},{y})"),
}
```

You can use `if let` for a single-match shortcut:

```rust
if let Message::Write(text) = msg {
    println!("{text}");
}
```

---

## Option<T>

Rust's safer version of null:

```rust
enum Option<T> {
    Some(T), // contains a value
    None,    // contains no value
}
```

Usage:

```rust
let v = Some(5);
let absent: Option<i32> = None;
```

Always check with `match`, `if let`, or `unwrap` / `expect`.

---

## Result<T, E>

Returned by operations that can fail for an explainable reason:

```rust
enum Result<T, E> {
    Ok(T),  // success, holds the value
    Err(E), // failure, holds error info
}
```

```rust
let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem: {error}"),
};
```

### The `?` operator

Propagate errors with a single character:

```rust
fn read_file() -> io::Result<String> {
    let mut f = File::open("hello.txt")?; // return Err early
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

| Shortcut | Behaviour |
|---|---|
| `.unwrap()` | Panic on None/Err |
| `.expect(msg)` | Panic with custom message |
| `.unwrap_or(v)` | Return `v` on None/Err |
| `.unwrap_or_else(f)` | Call `f()` on None/Err |
| `.ok()` | Convert Err→None |
| `.err()` | Convert Ok→None |

---

## let else — modern pattern matching

```rust
let Some(v) = opt else { return; }; // unwrap-or-early-return
let Ok(v) = result else { return Err(e); };
```
