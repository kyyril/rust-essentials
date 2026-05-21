# Exercise 1 — Variables and Mutability

> Complete the tasks below **without running the solution** until you've tried each one.

## Task 1.1 — Make the compiler happy

```rust
// TODO: fix the 3 lines below so this compiles
fn main() {
    let apples = 4;
    let more_apples = 10;
    more_apples = 20;
    println!("{apples} {more_apples}");
}
```

*Tips: which variables need `mut`? Do they run in the same scope?*

---

## Task 1.2 — Shadowing

Fix the code below. The goal is to create a variable `area` that holds the value
`40 * 20 = 800` using shadowing, then print it.

```rust
fn main() {
    let height = 40;
    let width = 20;
    // TODO: use shadowing to compute area as an integer
    let area = "???";
    println!("area = {area}");
}
```

*Hint: a `{ }` block that does not end with `;` is an expression.*

---

## Task 1.3 — Constants

Add a `const` declaration for a value that every valid Rust program must contain,
and print it:

```rust
fn main() {
    // TODO: add a const MIN_YEAR: u32 = 1900;
    println!("{}", MIN_YEAR);
}
```

Check: what happens when you omit the type annotation? Compile and read the error.

---

## Task 1.4 — Shadowing with type change

```rust
fn main() {
    let spaces = "    ";
    // TODO: shadow `spaces` with the number of spaces in it
    // (hint: call .len() on "")
    let spaces = ___;
    println!("count = {spaces}");
}
```

---

## Solution

<details>
<summary>Click to reveal</summary>

```rust
// 1.1
fn main() {
    let apples = 4;
    let mut more_apples = 10;  // ← add mut
    more_apples = 20;
    println!("{apples} {more_apples}");
}

// 1.2
fn main() {
    let height = 40;
    let width = 20;
    let area = {
        height * width  // expression, not statement
    };
    println!("area = {area}");
}

// 1.3
const MIN_YEAR: u32 = 1900;
fn main() {
    println!("{}", MIN_YEAR);
}

// 1.4
fn main() {
    let spaces = "    ";
    let spaces = spaces.len();  // shadowing also lets you change type
    println!("count = {spaces}"); // 4
}
```
</details>
