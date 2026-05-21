# Exercise 5 — Ownership

> Solve without copying the solution until you've thought through each task.

## Task 5.1 — Trace a move

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2;
    println!("{}", s3);
    // Q1: Which of s1 / s2 / s3 is valid at the println! line?
    // Q2: Trace value = " Ownership:
    //         · s1 → moved to s2 after assignment
    //         · s2 → moved to s3 after assignment
    //         · s3 ← only owner at the println! line
}
```

---

## Task 5.2 — Clone ownership, keep both

```rust
fn main() {
    let a = String::from("Rise");
    // TODO: assign b so that both a and b are valid after the assignment
    let ___ = a.___();
    println!("{a} {b}");
}
```

---

## Task 5.3 — Why did it panic?

```rust
fn main() {
    let result = divide(10, 0);
    println!("{}", result); // uncomment after writing divide()
}

// TODO: Fix divide() so this never panics.
// Return an i32 and print "Cannot divide by zero" from main when divisor is 0.
fn divide(a: i32, b: i32) -> ___ {
    // ── TODO ──
}
```

---

## Task 5.4 — Scope and drop

```rust
fn main() {
    {
        // TODO: create a String here
        let s = ___;
        println!("inner scope: {s}");
    }
    // ─ Q ── Was the String's memory freed? (Yes — drop is called automatically.)
}
```

---

## Task 5.5 — Compare Copy vs Move

```rust
fn main() {
    let a = 5;
    let b = a;      // Copy — i32 is Copy
    print_copied(a, b);
    // ── Q ── Replace the i32 literals with String::from("x") and fix the
    //           function so that it takes ownership and still compiles.
}

fn print_copied(x: i32, y: i32) {
    println!("{} {}", x, y);
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 5.2
let a = String::from("Rise");
let b = a.clone(); // deep copy, both stay alive
println!("{a} {b}");

// 5.3
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        // caller must check before calling,
        // or — better — use Option<i32>:
        return 0; // or: panic!("Cannot divide by zero");
    }
    a / b
}

// 5.4 — String is freed when the } closes.
// Note: you can use Rc<T> if you need shared ownership, but that is an
// advanced (smart pointer) topic.

// 5.5 — Option version is most idiomatic
fn print_owned() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    println!("{s1} {s2}"); // both still valid
}

// For the literal copy:
fn print_copied(a: String, b: String) { … } // both moved into fn
```
</details>
