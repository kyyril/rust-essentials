# 10 — OOP in Rust

## Overview

Rust is not a traditional object-oriented language. There are **no classes**, **no inheritance**, and **no constructors**. Instead, Rust achieves the same design goals through a combination of:

| OOP Goal | Rust Mechanism |
|---|---|
| Encapsulation | `pub` / private fields + modules |
| Polymorphism | **Traits** + **generics** or **trait objects** |
| Inheritance / Behaviour reuse | **Composition** + **trait impls** |
| Abstraction | **Trait** definitions |

Every concept from class-based OOP maps to a Rust feature. Programs are often safer and simpler.

---

## Traits — the heart of Rust "OOP"

A **trait** is the closest Rust equivalent of an interface. It defines a set of methods without implementation. Then one or more types *implement* the trait by providing the method bodies.

```rust
trait Drawable {
    fn draw(&self, x: u32, y: u32);
    fn area(&self) -> u32;
}
```

A trait definition is just the *contract* — no default data, no virtual table. Implementation is always explicit and opt-in.

### Implementing a trait for a struct

```rust
struct Circle { radius: u32, color: &'static str }

impl Drawable for Circle {
    fn draw(&self, x: u32, y: u32) { … }
    fn area(&self) -> u32 { 3 * self.radius * self.radius }
}
```

Multiple structs can implement the same trait, giving them shared behaviour **without** a common base class.

---

## Polymorphism — two flavours

### Static Dispatch (preferred)

Using **generics + trait bounds**, the compiler picks the concrete type at compile time (function is monomorphised):

```rust
fn print_area<T: Drawable>(item: &T) { println!("{}", item.area()); }
```

- Zero runtime cost (direct call, no v-table lookup).
- Binary may be slightly larger (one specialised copy per concrete type).
- Enable with `#[inline]` where it helps during LLVM optimisation.

### Dynamic Dispatch

Using a **trait object** (`&dyn Trait`), the choice of `draw()` implementation happens at **run time** via a hidden v-table:

```rust
fn draw_scene(items: &[&dyn Drawable]) {
    for item in items { item.draw(0, 0); }
}
```

Used when the concrete type is not known at compile time — e.g. a collection of mixed types.

---

## Encapsulation

Rust's visibility model is simple:

| Modifier | Visibility |
|---|---|
| (none) | Private to the current module |
| `pub(self)` | Explicitly private |
| `pub(crate)` | Visible inside the crate |
| `pub(super)` | Visible in the parent module |
| `pub(in path)` | Visible in a specified module |
| `pub` | Visible everywhere |

Private fields enforce encapsulation at the language level — there is no escaped private data:

```rust
pub struct BankAccount {
    pub owner:  &'static str, // fine, callers can read it
    balance:   u64,           // private: no extern access
}
```

Expose private fields through **public getter methods** only, just like Java or C#.

---

## Construction

Rust has no special `constructor` keyword. The convention is a `pub fn new(...) -> Self` associated function inside an `impl` block:

```rust
impl BankAccount {
    pub fn new(owner: &'static str, initial: u64) -> Self {
        Self { owner, balance: initial }
    }
}

let acct = BankAccount::new("Alice", 1_000);
```

If the initial value needs async work, use `async fn new(...)` — Rust handles it like any other function.

---

## Composition over inheritance

Rust has no subclassing. **Composition** is the idiomatic replacement:

```rust
struct Engine { horsepower: u32 }
impl Engine { fn start(&self) { … } }

struct Car { make: &'static str, engine: Engine }
impl Car {
    fn start(&self) {
        self.engine.start();     // delegate to Engine
    }
}
```

`Car` **has-a** `Engine` — the same graph you'd model with a UML aggregation arrow. Structs and `impl` blocks can be freely composed:

```rust
let c = Car { make: "Tesla", engine: Engine { horsepower: 500 } };
```

### Trait impls as mix-ins

Implement trait defaults across many types

---

## Default Trait Methods

Traits can supply fallback implementations for methods — analogous to default interface methods in Java 8+ or abstract base classes.

```rust
trait Greet {
    fn name(&self) -> &str;
    fn greet(&self) { println!("Hello, I'm {}", self.name()); } // default
}
```

Concrete types override only when they need to, but get a default "for free."

---

## Object-safety

A trait is **object-safe** when any method signature uses `Self` only as a receiver (not in a return type or generic parameter). Only object-safe traits can become **trait objects** (`&dyn Trait`):

| Allowed in trait object | NOT allowed |
|---|---|
| `&self`, `&mut self` | `fn new() -> Self` |
| method takes `&dyn Trait` | generic methods (`fn foo<T>()`) |

If a trait is not object-safe, prefer static dispatch (generics) instead.

---

## Summary

| OOP Concept | Rust equivalent |
|---|---|
| Class | `struct` + `impl` |
| Interface | `trait` |
| Subclassing | **no equivalent**; use composition |
| Constructor | `impl::new()` |
| Public / Private | `pub` / private-by-default |
| Virtual / override | trait method impl |
| Run-time polymorphism | `&dyn Trait` |
| Compile-time polymorphism | Generics + trait bounds |

Rust's approach — explicit, checked, opt-in — avoids fragile base-class antipatterns and diamond-problem complexity that plague deep class hierarchies.
