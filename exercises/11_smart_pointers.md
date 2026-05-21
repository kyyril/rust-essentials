# Exercise 11 — Smart Pointers

> Work through each task without running the solution first.

---

## Task 11.1 — Box a recursive type

Define a singly-linked list that is **recursive** (a node contains another node):

```rust
// TODO: fix this so it compiles — type must wrap the recursive field in Box
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = List::Cons(1, List::Cons(2, List::Nil));
    // use std::fmt to derive Display if you want pretty output
    println!("{:?}", list);
}
```

---

## Task 11.2 — Box<dyn Trait> heterogeneous vec

```rust
trait Animal {
    fn name(&self) -> &str;
}

struct Cat;
struct Dog;

// TODO: impl Animal for both Cat and Dog

fn main() {
    // TODO: create a `Vec<Box<dyn Animal>>` with 1 Cat and 1 Dog,
    //       then print each animal's name by calling .name() through the trait object.
}
```

---

## Task 11.3 — Rc<T> shared ownership

```rust
use std::rc::Rc;

fn main() {
    let rc1 = Rc::new("hello".to_string());
    let rc2 = Rc::clone(&___); // TODO: clone rc1
    let rc3 = Rc::clone(&___); // TODO: clone rc1 again

    println!(
        "rc1={} rc2={} rc3={} count={}",
        rc1, rc2, rc3, Rc::strong_count(&rc1)
    );
    // expect count=3

    drop(rc3);
    println!("after drop rc3 → {}", Rc::strong_count(&rc1));
    // expect 2
}
```

---

## Task 11.4 — Rc<RefCell<T>> interior mutability

```rust
use std::{cell::RefCell, rc::Rc};

fn main() {
    let shared = Rc::new(RefCell::new(0u32));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&___); // TODO: clone shared again

    *a.borrow_mut() += 10;
    *b.borrow_mut() *= 2;

    println!("shared = {}", shared.borrow()); // expect 20
}
```

---

## Task 11.5 — Arc<T> across threads

```rust
// TODO: fix the code below and verify it works
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..2 {
        handles.push(thread::spawn(|| {
            println!("thread {i}: {:?}", data); // TODO: clone data into the closure
        }));
    }
    for h in handles { h.join().unwrap(); }
}
```

---

## Task 11.6 — Mutex<T> shared counter

Write a program that:

1. Creates `Arc<Mutex<u32>>` set to `0`.
2. Spawns **8 threads**.
3. Each thread increments the counter **1000 times**.
4. Print the final count — must be **8000**.

---

## Task 11.7 — Choosing the right smart pointer

Fill in the blank:

```rust
// Scenario                             → Smart pointer
// ──────────────────────────────────     ──────────────────
// Heap-allocate a recursive type        → Box<___>
// Single owner, trait object            → Box<dyn ___>
// Single-thread shared ownership        → Rc<___>
// Cross-thread shared, read-only        → Arc<___>
// Single-thread interior mutability     → RefCell<___>
// Cross-thread interior mutability      → Arc<Mutex<___>>
```

---

## Task 11.8 — Break a reference cycle with Weak

```rust
use std::{cell::RefCell, rc::{Rc, Weak}};

struct Node {
    name:   String,
    next:   RefCell<Option<Weak<Node>>>,
}

fn main() {
    let a = Rc::new(Node { name: "A".into(), next: RefCell::new(None) });
    let b = Rc::new(Node { name: "B".into(), next: RefCell::new(None) });

    // TODO: link a → b using Weak (no cycle)
    // then print `b`'s strong count — must still be 1
}
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 11.1
enum List { Cons(i32, Box<List>), Nil }

// 11.2
trait Animal { fn name(&self) -> &str; }
struct Cat; struct Dog;
impl Animal for Cat { fn name(&self) -> &str { "cat" } }
impl Animal for Dog { fn name(&self) -> &str { "dog" } }
fn main() {
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    for a in &zoo { println!("{}", a.name()); }
}

// 11.3
let rc2 = Rc::clone(&rc1); let rc3 = Rc::clone(&rc1);
// count=3 → drop(rc3) → 2

// 11.4
let b = Rc::clone(&shared); // result 20

// 11.5
for i in 0..2 {
    let d = Arc::clone(&data);
    handles.push(thread::spawn(move || println!("thread {i}: {:?}", d)));
}

// 11.6 — final count = 8000
// 11.7 — T / Trait / T / T / T / T
// 11.8
*b.next.borrow_mut() = Some(Rc::downgrade(&b));
println!("{}", Rc::strong_count(&b)); // 1
```
</details>
