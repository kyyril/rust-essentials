# Exercise 2 — Data Types

> Try solving each task before checking the solution.

## Task 2.1 — Integer overflow

```rust
fn main() {
    let x: u8 = 255;
    // What do you think happens when you uncomment this?
    // let y = x + 1;
    // In debug builds it panics; in release builds it wraps to 0.
    println!("x = {x}");
}
```

Run both a debug build and a release build and note the difference.

---

## Task 2.2 — Numeric type conversion

```rust
fn main() {
    let a = 15;
    let b = 4.2;
    // TODO: compute a / b as f64 and print with 2 decimal places.
    // Hint: write `a as f64 / b`
}
```

---

## Task 2.3 — Tuple destructuring

```rust
fn main() {
    let tup = (500, 6.4, 'Z');
    // TODO: destructure tup into int_val, float_val, char_val
    // then compute int_val * float_val and print it
}
```

---

## Task 2.4 — Array sum

```rust
fn main() {
    // TODO: declare an array [10; 5] and compute the sum using a loop
    let numbers = [___; ___];
    let mut sum = 0;
    // ...
    println!("sum = {sum}");
}
```

---

## Task 2.5 — Booleans and chars

```rust
fn main() {
    // TODO: determine whether 'A' is uppercase.
    // Hint: use char.is_uppercase()
    let c = 'A';
    let is_upper = ___;
    println!("{c} is uppercase? {is_upper}");
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 2.2
fn main() {
    let a = 15;
    let b: f64 = 4.2;
    let quotient = a as f64 / b;
    println!("{quotient:.2}"); // 3.57
}

// 2.3
fn main() {
    let tup = (500, 6.4, 'Z');
    let (int_val, float_val, char_val) = tup;
    println!("product = {}", int_val as f64 * float_val);
}

// 2.4
fn main() {
    let numbers = [10; 5];          // [10, 10, 10, 10, 10]
    let mut sum = 0;
    for n in numbers.iter() {
        sum += n;
    }
    println!("sum = {sum}"); // 50
}

// 2.5
fn main() {
    let c = 'A';
    let is_upper = c.is_uppercase();
    println!("{c} is uppercase? {is_upper}");
}
```
</details>
