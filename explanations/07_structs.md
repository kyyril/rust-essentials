# 7 — Structs

## Overview

A struct is a **named type** that groups multiple related fields together. Unlike a tuple, every field is named, making the meaning of each value self-documenting.

---

## Defining a Struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

Create an **instance** by providing a value for each field:

```rust
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};
```

Access fields with **dot syntax**:

```rust
println!("email: {}", user1.email);
let user2 = user1;        // MOVE — all String fields moved
```

---

## Struct Update Syntax

Copy selected fields from another instance using `..`:

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user1          // fills remaining fields from user1
};
```

Fields that haven't changed must be available — `user1.username` and `user1.active` are copied here.

---

## Tuple Structs

When naming the struct matters more than the field names:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Tuple structs still have a type, so `Color` and `Point` cannot be mixed.

---

## Unit Structs

A struct with no fields — useful as **marker types**:

```rust
struct AlwaysEqual;

let x = AlwaysEqual; // no parens required
```

---

## Methods — impl blocks

Use `impl` blocks to define functions that operate on instances of a struct:

```rust
impl Rectangle {
    fn area(&self) -> u32 {       // &self = borrow
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
```

Method parameters:

| Notation | Meaning |
|---|---|
| `&self` | Borrow immutably — most common |
| `&mut self` | Borrow mutably |
| `self` | Take ownership (consuming method) |

### Associated functions

Functions inside `impl` that don't take `self` are **not methods**:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {     // constructor
        Self { width, height }
    }
}

let r = Rectangle::new(30, 50); // called with ::
```

---

## Deriving common traits

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
```

`Debug` lets you use `{:?}` in `println!` for quick developer printing.

### Derived vs custom traits

| Trait | Auto-derived | Why write impl |
|---|---|---|
| `Debug`, `Clone`, `PartialEq` | Yes | Formatting, deriving |
| `Display`, `From` | No | Human-readable output, conversions |
