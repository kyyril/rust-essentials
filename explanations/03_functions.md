# 3 — Functions

## Overview

Functions are Rust's primary unit of code organisation. They are declared with `fn` and use the `snake_case` convention by default.

---

## Syntax

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
    return_value         // last expression (no ;) = return
}
```

### Statements vs Expressions

| | Statements | Expressions |
|---|---|---|
| Ends with `;` | Yes | No |
| Returns a value | No | Yes |

```rust
let x = {
    let y = 10;
    y + 1   // expression → becomes 11 (no ;)
};          // statement

fn add(a: i32, b: i32) -> i32 { a + b } // whole fn body = expression
```

### Return Values

There are two equivalent ways to return a value:

```rust
// implicit: last expression, no semicolon
fn add(a: i32, b: i32) -> i32 { a + b }

// explicit: return keyword
fn add(a: i32, b: i32) -> i32 { return a + b; }
```

The implicit style is preferred; `return` is reserved for early exits.

### Multiple Return Values

Use a tuple:

```rust
fn divide(a: f64, b: f64) -> (f64, &'static str) {
    if b == 0.0 { return (0.0, "div by zero"); }
    (a / b, "ok")
}
```

### Functions Returning Ownership

```rust
fn gives() -> String {
    String::from("hello")  // moved to caller
}
```

### Functions Taking Borrows

```rust
fn borrow_len(s: &String) -> usize { s.len() }
// Caller keeps ownership: borrow_len(&s)
```

## Main

`fn main()` is the program's entry point. It returns `()` — it never panics or returns a value.

---

## Patterns

| Pattern | When to use |
|---|---|
| `panic!` | Programmer error; stop execution |
| `Option<T>` | Nullable-like values |
| `Result<T, E>` | Fallible operations |
| `Result` + `?` | Propagate errors cleanly upward |
