# 2 — Data Types

## Overview

Every value in Rust has a type. There are two broad categories:

| Category | Description |
|---|---|
| **Scalar** | Represents a single value |
| **Compound** | Groups multiple values |

Rust is **statically typed** — the compiler knows every type at compile time.

---

## Scalar Types

### Integer Literals

| Literal | Effect |
|---|---|
| `Decimal` | `982_222` |
| `Hex` | `0xff` |
| `Octal` | `0o77` |
| `Binary` | `0b1010` |
| `Byte` (u8 only) | `b'A'` |

### Integer Types

| Type | Min Signed | Max Signed | C equivalent |
|---|---|---|---|
| `i8` | −128 | 127 | `char` (signed) |
| `i16` | −32 768 | 32 767 | |
| `i32` | −2 B | 2 B − 1 | `int` |
| `i64` | −9 E | 9 E | |
| `i128` | −34 E | 34 E | |
| `isize` | depends on arch | | `ssize_t` |
| `u8`..`u128` / `usize` | 0 | max minus 1 | |

Overflow **panics in debug builds** and **wraps in release builds** — use `wrapping_add()` etc. to be explicit.

### Float Types

| Type | Bits | C equivalent |
|---|---|---|
| `f32` | 32 | `float` |
| `f64` | 64 | `double` (default) |

### Boolean

```rust
let t: bool = true;
```

### Character

A `char` is a **4-byte Unicode scalar value** — not just ASCII.

```rust
let c: char = '🔥';  // emoji is a single char
'π';                 // mathematical symbol
'\n';                // escape sequence
```

### Numeric Operations

Rust supports `+`, `-`, `*`, `/`, `%` — the set is intentionally simple. Avid C programmers will notice integer division truncates toward zero.

---

## Compound Types

### Tuples

Fixed-length collections of possibly different types.

```rust
let t: (i32, f64, char) = (500, 6.4, 'x');
println!("first={}", t.0);         // index access
let (w, _, z) = t;                 // destructure
```

**Unit tuple** `()` is the empty tuple — Rust's equivalent of the empty type `void`.

### Arrays

Fixed-length, all elements same type.

```rust
let nums = [1, 2, 3, 4, 5];    // inferred
let zeros = [0u8; 3];           // [0, 0, 0]
let months = ["Jan", "Feb", …];
```

Access with `array[index]` — **bounds-checked at runtime**, panics on OOB.

Unlike C, Rust has **no pointer arithmetic** on arrays by default.

| Type | Mutability | Heap | Length |
|---|---|---|---|
| `[T; N]` (array) | Yes | No | Fixed |
| `Vec<T>` | Yes | Yes | Dynamic |
| `[T]` (slice) | no | — | Dynamic |
