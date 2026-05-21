# 4 — Control Flow

## Overview

Rust provides the standard control flow constructs (if, loops, for) with some idiomatic additions: loop-as-expression, labelled loops, `match` for exhaustive pattern matching, and `for` as the preferred iterator.

---

## if

- No parentheses required around the condition.
- `if` is an **expression**, not a statement — it returns a value.
- Every arm must return the same type.

```rust
let x = if condition { 5 } else { 6 };
```

Conditions must be `bool` type — Rust does **not** treat numbers as booleans (unlike C).

---

## Loops

### loop — infinite loop

```rust
loop {
    // runs forever unless broken
    break;             // exit loop
    break value;       // loop is an expression → returns value
}
```

### while — conditional loop

```rust
while condition { … }
```

`while` checks the condition before every iteration. For counting use `for` instead (it's less error-prone).

```rust
// Prefer:
for i in 1..=5 { … }       // inclusive range: 1,2,3,4,5
// Over:
while i <= 5 { … }
```

### for — iterate over collections

The most common loop in Rust. Works with any type implementing `Iterator`.

```rust
for item in collection.iter() { … }
for item in collection.into_iter() { … } // takes ownership
for item in collection.iter_mut()  { … } // mutable borrow
```

---

## match — exhaustive pattern matching

`match` must cover **every possible value** — the compiler enforces this.

```rust
match value {
    Pattern1 => result1,
    Pattern2 => result2,
    _ => default,          // wildcard: catch-all
}
```

### Match guards — extra conditions inside match arms

```rust
match n {
    x if x < 0 => "negative",
    x if x == 0 => "zero",
    _ => "positive",
}
```

(Local `if let` is desugared to `match` under the hood.)

---

## if let — single-pattern shorthand

```rust
// Verbose
match opt {
    Some(v) => println!("{v}"),
    None => {}
}

// Concise
if let Some(v) = opt {
    println!("{v}");
}
```

Use `if let` when you only care about **one** pattern — `match` when you want exhaustive handling.

### let else — inverse match

```rust
let Some(v) = opt else { return }; // forces fallthrough on None
```

---

## Range operators

| Syntax | Meaning |
|---|---|
| `a..b` | `a` (inclusive) to `b` (exclusive) |
| `a..=b` | `a` to `b` (both inclusive) |
| `..b` | from start to `b` (excl.) |
| `a..` | from `a` to end |
