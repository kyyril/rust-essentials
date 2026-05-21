# 1 — Variables and Mutability

## Overview

Rust's approach to variables is fundamentally different from most other languages. By default, variables are **immutable** — once given a value, they cannot be changed.

This design choice promotes both **correctness** and **concurrency safety**. When you read a variable you know it will always hold the same value, distributed across threads without locks.

---

## Key Concepts

### Immutability (the default)

```rust
let x = 5;
x = 6; // ❌ compile error: cannot assign twice to immutable variable
```

The compiler catches this mistake before your program even runs.

### Mutability

```rust
let mut score = 0;
score = 10; // ✅ OK
```

Use `mut` only when you truly need mutation. Prefer immutable by default.

### Shadowing

Rust lets you declare a new variable with the same name, "shadowing" the old one:

```rust
let x = 5;
let x = x + 1;   // new binding, still immutable
let x = "hello"; // can even change type
```

Shadowing is useful for transforming a value through a series of steps while keeping context readable.

### Constants

```rust
const MAX_POINTS: u32 = 100_000;
```

- Immutable by definition (cannot use `mut`).
- Naming convention: `SCREAMING_SNAKE_CASE`.
- Type must be specified.
- Inlined by the compiler — no memory address.
- Set at compile time, not runtime.

### Why this matters

| Feature | Benefit |
|---|---|
| Immutable by default | Fewer bugs, easier reasoning |
| Mutability is explicit | Clear intent |
| Shadowing | Safe value transformation |
| Constants | Guaranteed single-evaluation |

---

## Common Pitfalls

### Forgetting `mut`

```rust
let name = "Alice";
name = "Bob"; // ❌ forgot mut
// Fix: let mut name
```

### Unnecessary `mut`

```rust
let mut x = 5;
x = x * 2;
// idiomatic: let x = x * 2;  (shadowing, no mut needed)
```

### Const vs static

- `const` — inlined everywhere; no storage.
- `static` — a single memory location (only for globals).
- For beginners: stick with `const` unless you need a mutable global.
