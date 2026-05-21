# Exercise 7 — Structs

## Task 7.1 — Create and print a struct

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // TODO: create a Rectangle with width=30 and height=50,
    //       then print it using {:?}
}
```

---

## Task 7.2 — Write a constructor

Add an `impl` block giving `Rectangle` a `fn new(width, height) -> Self`

```rust
impl Rectangle {
    // TODO: fn new(…) — sets width and height
}

fn main() {
    let r = Rectangle::new(4, 6);
    println!("area = {}", r.area());
}
```

---

## Task 7.3 — Implement `area` and `perimeter` methods

```rust
impl Rectangle {
    // TODO: fn area(&self) → width * height
    // TODO: fn perimeter(&self) → 2 * (width + height)
}
```

---

## Task 7.4 — Sequence matching inside a method

```rust
impl Rectangle {
    // TODO: fn can_hold(&self, other: &Rectangle) → bool
    //          returns true when self can completely enclose `other`
}

fn main() {
    let big   = Rectangle::new(100, 200);
    let small = Rectangle::new(10,  20);
    println!("{}", big.can_hold(&small));  // expect true
    println!("{}", small.can_hold(&big));  // expect false
}
```

---

## Task 7.5 — Tuple struct → named-struct conversion

```rust
struct Color(i32, i32, i32);

// TODO: add a method .is_grayscale() that returns true when r == g == b
fn main() {
    let gray = Color(128, 128, 128);
    println!("{}{}", "gray".repeat(1), if gray.is_grayscale() { " ✓" } else { " ✗" });
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 7.1
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("{r:?}");
}

// 7.2 + 7.3 + 7.4 + 7.5
impl Rectangle {
    fn new(w: u32, h: u32) -> Self { Self { width: w, height: h } }
    fn area(&self)    -> u32 { self.width * self.height }
    fn perimeter(&self) -> u32 { 2 * (self.width + self.height) }
    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Color {
    fn is_grayscale(&self) -> bool { self.0 == self.1 && self.1 == self.2 }
}
```
</details>
