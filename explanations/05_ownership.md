# 5 — Ownership

## Overview

Ownership is the most distinctive feature of Rust's memory model. It lets Rust guarantee **memory safety without a garbage collector**.

---

## The Three Ownership Rules

These rules enforced at compile time by the compiler:

1. Each value has exactly **one owner**.
2. When the owner goes out of scope, the value is **automatically dropped**.
3. There can be only **one owner at a time**.

---

## Scope

A scope is the range within which an item is valid. When the `}` closing brace is reached, the item goes out of scope and Rust calls `drop` (an implicit `free`).

```rust
fn main() {
    let s = String::from("hello"); // ← s is valid
    // use s …
} // ← s goes out of scope → drop happens automatically
```

No need to manually `free()` — Rust handles it, no leaks, no double-frees.

---

## Moves

For types that own heap data (like `String`), **assignment transfers ownership**:

```rust
let s1 = String::from("hello");
let s2 = s1;        // s1 is MOVED to s2
// println!("{s1}"); // ❌ compile error: s1 was moved
println!("{s2}");    // ✅ OK
```

After a move, the original variable is **invalid** — using it is a compile error.

### Why not always copy? — Deep vs Shallow copy

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy — heap data duplicated
println!("{s1} {s2}"); // both valid
```

`clone()` is explicit and expensive — making the caller pay for it. Most of the time you don't need two copies.

---

## Copy types — the stack-only exception

Types that live entirely on the stack can be **bitwise-copied** instantaneously:

| Copy | Not Copy |
|---|---|
| All integers & floats | `String` |
| `bool`, `char` | `Vec<T>` |
| `&T` references | `Box<T>` |
| tuples of copy types | structs containing non-Copy fields |

Integer types implement the `Copy` trait automatically:

```rust
let x = 5;
let y = x;     // copy, not move
println!("{x} {y}"); // both valid
```

---

## Ownership and Functions

Ownership follows the same rules inside functions as it does in `main()`:

```rust
let s = String::from("hello");
takes_ownership(s); // s MOVED
// s is no longer valid
```

Returning ownership is the only way to use a heap value after it has been moved:

```rust
let s2 = gives_ownership();
let s3 = takes_and_gives_back(s2);
```

Returning ownership every time is tedious — **borrowing & references** (next topic) addresses this.

---

## Common Pitfalls

| Mistake | Correct approach |
|---|---|
| Using moved variable | Clone or pass reference |
| Forgetting `clone()` when needed | Explicitly call `.clone()` |
| Treating all types as Copy | Check `T: Copy` in docs |
