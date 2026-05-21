# Exercise 6 — Borrowing and References

> Solve each task before peeking at the solutions.

## Task 6.1 — Borrow, don't move

The code below does not compile because `report()` takes ownership.
Fix it by changing the function signature and the call site to use references.

```rust
fn report(s: String) -> &str {
    &s
}

fn main() {
    let msg = String::from("Rust");
    let report = report(msg);
    println!("{report}");
}
```

---

## Task 6.2 — Multiple reads, one write in sequence

```rust
// TODO: add MUT to the minimum number of lines so this compiles
fn main() {
    let mut s = String::from("hello");
    // println!("{}, {}", s, s);         // two immutable borrows
    let r = &mut s;
    println!("{r}");
    println!("{}", s);                  // s used again here
}
```

---

## Task 6.3 — Pass a mutable reference into a function

Write `append_world` that takes a `&mut String` and pushes `", world!"` onto it.

```rust
fn main() {
    let mut greeting = String::from("hello");
    // TODO: call append_world so greeting reads "hello, world!"
    println!("{greeting}");
}

fn append_world(s: &mut String) {
    // ── TODO ──
}
```

---

## Task 6.4 — Slice length

```rust
fn main() {
    let sentence = String::from("hello beautiful world");
    // TODO: use a string slice to get the first 2 words ("hello beautiful")
    let first_two = ___;
    println!("'{}' has {} chars", first_two, first_two.len());
}
```

---

## Task 6.5 — Slice from an array

```rust
fn main() {
    let nums = [1, 3, 5, 7, 9];
    // TODO: take a slice of the middle three elements
    //       and return it from main(); then print it.
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 6.1 — borrow instead of move
fn report(s: &String) -> &str { s }
// or simply fn report(s: &str) -> &str { s }
fn main() {
    let msg = String::from("Rust");
    let report = report(&msg);
    println!("{report}");
}

// 6.2 — only &mut s between the two prints
fn main() {
    let mut s = String::from("hello");
    println!("{}, {}", s, s);
    let r = &mut s;
    println!("{r}");
    println!("{s}");
}

// 6.3
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

// 6.4
fn main() {
    let sentence = String::from("hello beautiful world");
    let first_two = &sentence[0..14];  // 14 = "hello beautiful"
    // Better: find indices using .find(" ")
}

// 6.5
fn main() {
    let nums = [1, 3, 5, 7, 9];
    let slice = &nums[1..4];        // [3, 5, 7]
    println!("{:?}", slice);
}
```
</details>
