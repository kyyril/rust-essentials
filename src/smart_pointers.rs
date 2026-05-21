// Smart Pointers in Rust
// Run with: cargo run --bin smart_pointers

// =========================================================================
// 1. Box<T> — heap allocation + recursive types + trait objects
// =========================================================================
fn box_demo() {
    // Box places a value on the heap; the pointer itself lives on the stack.
    // Box owns its value — when the Box is dropped, the heap value is freed.
    let b = Box::new(42_i32);
    println!("Boxed i32 = {}", b); // dereferences via Deref

    // --- Recursive type problem ---
    // This would fail to compile (infinite size):
    //   enum List { Cons(i32, List), Nil }
    // Box fixes it because Box<List> has a known size (one pointer)
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("list = {:?}", list);

    // --- Trait objects ---
    // Box<dyn Trait> = "safe fat pointer": data ptr + vtable ptr
    trait Speak {
        fn speak(&self) -> &str;
    }
    struct Cat;
    struct Dog;
    impl Speak for Cat { fn speak(&self) -> &str { "meow" } }
    impl Speak for Dog { fn speak(&self) -> &str { "woof" } }

    let zoo: Vec<Box<dyn Speak>> = vec![Box::new(Cat), Box::new(Dog)];
    for animal in &zoo {
        println!("{}", animal.speak()); // dynamic dispatch
    }
}

// =========================================================================
// 2. Rc<T> — reference-counted single-thread shared ownership
// =========================================================================
use std::rc::Rc;

fn rc_demo() {
    // Rc<T> keeps a count of references.
    // When the count hits 0, the heap value is dropped.
    // ⚠️ Rc is NOT thread-safe — use Arc for shared state across threads.
    let rc_a = Rc::new(String::from("Rust"));
    let rc_b = Rc::clone(&rc_a); // clone increments the weak count
    let rc_c = Rc::clone(&rc_a);

    println!(
        "a={} b={} c={}  strong_count={}",
        rc_a, rc_b, rc_c,
        Rc::strong_count(&rc_a) // 3
    );

    drop(rc_c); // decrement
    println!("after drop(c) → {}", Rc::strong_count(&rc_a)); // 2

    // "Downgrade" to Weak — doesn't prevent deallocation
    let weak = Rc::downgrade(&rc_a);
    drop(rc_b);
    println!("after drop(b) → {}", Rc::strong_count(&rc_a)); // 1

    drop(rc_a); // value freed (strong count → 0)
    // weak.upgrade() returns Option — None when value is gone
    println!("weak still alive? {}", weak.upgrade().is_some()); // false
}

// =========================================================================
// 3. Arc<T> — atomic reference count (thread-safe shared ownership)
// =========================================================================
use std::sync::Arc;
use std::thread;

fn arc_demo() {
    let shared = Arc::new(vec![1u32, 2, 3, 4, 5]);

    let mut handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            println!("thread {i}: {:?}", data);
        });
        handles.push(handle);
    }
    for h in handles { h.join().unwrap(); }
    println!("Arc strong count after joins: {}", Arc::strong_count(&shared)); // 1
}

// =========================================================================
// 4. RefCell<T> — interior mutability (run-time borrow checking)
// =========================================================================
use std::cell::RefCell;

fn refcell_demo() {
    // RefCell<T> allows mutation through an immutable reference,
    // enforcing borrow rules at RUN TIME instead of compile time.
    // Panics if you violate the rules:
    //   · two simultaneous mutable borrows
    //   · mutable borrow while immutable borrows exist

    let data = RefCell::new(vec![1, 2, 3]);

    // Borrow immutably
    {
        let borrow = data.borrow();              // shared borrow
        println!("borrow[0] = {}", borrow[0]);   // OK
    } // borrow dropped here

    // Borrow mutably
    {
        let mut borrow = data.borrow_mut();      // exclusive borrow
        borrow.push(4);
        println!("after push: {:?}", borrow); // [1,2,3,4]
    }

    // Interior mutability with multiple owners: Rc<RefCell<T>>
    let shared = Rc::new(RefCell::new(100));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() += 50; // mutate through one handle
    println!("shared value: {}", b.borrow());   // 150 — both see the change
}

// =========================================================================
// 5. Mutex<T> — mutual exclusion (thread-safe interior mutability)
// =========================================================================
use std::sync::Mutex;
// std::sync::RwLock — many concurrent readers OR one exclusive writer

fn mutex_demo() {
    // Mutex<T> provides interior mutability with mutual exclusion.
    // Access via .lock() → guards the lock; guard is dropped (unlock) at end of scope.
    let counter = Mutex::new(0u32);
    let arc     = Arc::new(counter);

    let mut handles = vec![];
    for _ in 0..4 {
        let data = Arc::clone(&arc);
        handles.push(thread::spawn(move || {
            // lock() returns a MutexGuard → derefs to &mut u32
            let mut num = data.lock().unwrap();
            *num += 1;
            // guard dropped here → unlock
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("final counter = {}", arc.lock().unwrap()); // 4
}

// =========================================================================
// 6. Choosing the right smart pointer
// =========================================================================
fn choosing_demo() {
    println!(
        "\n=== Decision guide ===\n\
         Box<T>        → single owner, value on heap / recursive type / trait object\n\
         Rc<T>         → multiple owners, single-thread read-only\n\
         Arc<T>        → multiple owners, cross-thread read-only\n\
         Rc<RefCell<T>>→ multiple owners, single-thread interior mutability\n\
         Arc<Mutex<T>> → multiple owners, cross-thread interior mutability"
    );
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    println!("=== 1. Box<T> ===");
    box_demo();

    println!("\n=== 2. Rc<T> ===");
    rc_demo();

    println!("\n=== 3. Arc<T> ===");
    arc_demo();

    println!("\n=== 4. RefCell<T> ===");
    refcell_demo();

    println!("\n=== 5. Mutex<T> ===");
    mutex_demo();

    choosing_demo();
}
