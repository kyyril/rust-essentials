# 6 — Borrowing and References

## Overview

References allow you to **borrow** a value without taking ownership. This is the primary mechanism Rust uses to let you pass data around without triggering move semantics.

---

## Reference Syntax

```rust
&val  // immutable reference
&mut val // mutable reference
```

| | Syntax | Ownership | Mutation |
|---|---|---|---|
| Immutable reference | `&T` | borrow | read-only |
| Mutable reference | `&mut T` | borrow | read-write |

---

## Immutable References

```rust
fn len(s: &String) -> usize { s.len() }

let s = String::from("hello");
let l = len(&s); // borrowed — s still valid in caller
println!("{s}"); // ✅
```

### Multiple immutable borrows are allowed

```rust
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}"); // fine
```

---

## Mutable References

```rust
fn add_greeting(s: &mut String) {
    s.push_str(", world!");
}

let mut s = String::from("hello");
add_greeting(&mut s);
println!("{s}"); // "hello, world!"
```

### Only ONE mutable borrow at a time

```rust
let r1 = &mut s;
let r2 = &mut s;  // ❌ compile error: second mutable borrow
```

This prevents data races at compile time — no runtime cost.

---

## The Borrow Checker — rules summary

| Rule | Detail |
|---|---|
| Any number of `&T` | Multiple readers are fine simultaneously |
| Exactly one `&mut T` | Writer needs exclusive access |
| References must be valid | No dangling references |
| `&mut T` cannot alias | Writer alone |

### NLL (Non-Lexical Lifetimes)

The compiler applies NLL: the lifetime of a borrow ends at its **last use**, not at the closing `}`:

```rust
let mut s = String::from("abc");
let r = &s;
println!("{r}");     // r's last use → borrow ends here
s.push_str("!");     // ✅ borrow already released
```

---

## Dangling References

Rust **never allows** a reference to outlive the data it points to:

```rust
let r;
{
    let x = 5;
    r = &x;        // ❌ x is dropped, r would dangle
}
// Compiler catches this: "x does not live long enough"
```

---

## Slices — a view into a sequence

A slice is a **reference type** that borrows a contiguous portion of a collection:

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // &str — borrowed portion
let world = &s[6..11];
println!("{hello} {world}");
```

### String literals are slices

```rust
let s: &str = "hello world"; // type is &str = string slice
```

---

## Taking a reference vs taking ownership

| When | Pass |
|---|---|
| Only reading | `&value` |
| Mutating without taking ownership | `&mut value` |
| Need caller to lose the value | move it |
