# 11 — Smart Pointers

## Overview

A **smart pointer** is a type that acts like a pointer *and* carries extra metadata or behaviour. Unlike raw C pointers they manage memory automatically — they `drop` their value when the last reference goes out of scope.

---

## Box<T> — owned heap allocation

`Box<T>` is the simplest smart pointer. It places a **single value on the heap** and owns it.

```
Stack                 Heap
┌─────────┐         ┌──────────┐
│  b ────┼──────▶  │  42_i32  │
│  owner: │         └──────────┘
└─────────┘
```

```rust
let b = Box::new(42_i32);
println!("{}", b); // Box auto-derefs via Deref
drop(b);            // heap value freed here
```

### When to use `Box`

| Use-case | Why |
|---|---|
| Large data you don't want on the stack | Bigger than ~8 KiB |
| Trait objects | `Box<dyn Trait>` has uniform size |
| Recursive types | Fixes the infinite-size compiler error |
| Closures / async | `FnBox` allocation |

```rust
// Box<dyn Speak> — one vector holding mixed concrete types
let zoo: Vec<Box<dyn Speak>> = vec![Box::new(Cat), Box::new(Dog)];
```

### Recursive types

Without `Box`, this is infinite-size:
```rust
enum List { Cons(i32, List), Nil }  // ❌ infinite size
```

With `Box` it's one pointer wide:
```rust
enum List { Cons(i32, Box<List>), Nil }  // ✅ known size
```

---

## Rc<T> — reference-counted ownership (single-thread)

`Rc<T>` lets **multiple owners** share the same heap-allocated value.

```
┌──────┐         ┌──────┐         ┌──────┐
│ rc_a │ ──┐     │ data │     ┌──▶ 42_u8│
└──────┘   └────▶│      │─────┘   └──────┘
┌──────┐   ┌─────┘      │
│ rc_b │ ──┘            │ strong_count = 2
└──────┘
```

```rust
let rc_a = Rc::new(String::from("hello"));
let rc_b = Rc::clone(&rc_a);
println!("count = {}", Rc::strong_count(&rc_a)); // 2
```

`strong_count` — normal owners. `weak_count` — `Weak` handles (don't prevent deallocation).

### Weak<T> — non-owning weak reference

A `Weak<T>` points to the value without incrementing the strong count — useful for **breaking reference cycles** (parent → child → parent):

```rust
let weak = Rc::downgrade(&rc_a);
println!("alive? {}", weak.upgrade().is_some()); // true while strong count > 0
```

### Rc rules

| Rule | Detail |
|---|---|
| Zero errors, run-time panic-free | Borrow rules not enforced |
| Only single-thread | `Rc` is NOT `Send` / `Sync` |
| `Rc::clone` | Increments count — cheap |
| `drop(Rc)` | Decrements — frees when count hits 0 |

---

## Arc<T> — atomic reference count (thread-safe)

`Arc<T>` = **Atomic Reference Counted**. Identical API to `Rc<T>` but uses atomic operations so it's safe to share across threads.

```
Thread 1 ──┐
           │   Arc<T>  ──▶  shared data (heap)
Thread 2 ──┤
           │   Arc<T>
```

```rust
let shared = Arc::new(vec![1, 2, 3]);
for _ in 0..4 {
    let s = Arc::clone(&shared);
    thread::spawn(move || println!("{:?}", s));
}
```

### Arc rules

| Rule | Detail |
|---|---|
| `Send` + `Sync` | Safe to share across threads |
| Atomic ops | Slightly slower than `Rc` per op |
| Use when | Data is accessed from multiple OS threads |

**Mutex / RwLock pair**: for shared *mutable* state, combine `Arc<Mutex<T>>` (writers exclusive) or `Arc<RwLock<T>>` (many readers, one writer).

---

## RefCell<T> — interior mutability (single-thread)

`RefCell<T>` enforces **borrow rules at run time** — not at compile time. It has no compile-time borrow checker. Violations panic at run time.

```
RefCell<T>
   │
   ├── .borrow()      → Ref<T>  (multiple simultaneous)
   └── .borrow_mut()  → RefMut<T> (exclusive)
```

```rust
let data = RefCell::new(vec![1, 2, 3]);

// Multiple immutable borrows — ok
let a = data.borrow();
let b = data.borrow();
// println!("{}, {}", a[0], b[0]);

// Mutable borrow — panics if any Ref is still alive
data.borrow_mut().push(4);
```

### When to use `RefCell`

| Situation | Use |
|---|---|
| Caching / memoisation | Interior reference cycles |
| Per-elem mutability in `Vec<T>` | `Vec<RefCell<T>>` |
| Single-thread interior mutability | `Rc<RefCell<T>>` |

### `Rc<RefCell<T>>` — shared, interior-mutable data (single-thread)

```rust
let shared = Rc::new(RefCell::new(100));
let a = Rc::clone(&shared);
let b = Rc::clone(&shared);
*a.borrow_mut() += 50; // mutate through a
println!("{}", b.borrow()); // 150 — both see the change
```

---

## Mutex<T> — mutual exclusion (thread-safe interior mutability)

`Mutex<T>` lets multiple threads **mutate shared state safely** — only one thread holds the lock at a time.

```rust
let counter = Mutex::new(0u32);
let arc     = Arc::new(counter);

thread::spawn(move || {
    let mut num = arc.lock().unwrap(); // blocks until lock acquired
    *num += 1;                         // guard auto-unlocks on drop
});
```

### Guard pattern

`lock()` returns a `MutexGuard<'_, T>` that auto-unlocks at end of scope:

```rust
{
    let guard = m.lock().unwrap(); // lock acquired
    *guard += 1;                   // modify through guard
} // ← guard dropped here → lock released
```

### RwLock<T>  

Same as `Mutex`, but allows **concurrent readers** (multiple `read()` guards) and blocks writers until all readers finish.

---

## Comparison table

| Smart Pointer | Holds | Thread-safe | Interior mut | Multiple owners |
|---|---|---|---|---|
| `Box<T>` | 1 value, heap | No | No | No |
| `Rc<T>` | 1 value, heap | No | No | Yes |
| `Arc<T>` | 1 value, heap | Yes | No | Yes |
| `RefCell<T>` | 1 value, stack/heap | No | Yes (run-time) | No |
| `Rc<RefCell<T>>` | 1 value, heap | No | Yes (run-time) | Yes |
| `Mutex<T>` | 1 value, heap | Yes | Yes (run-time) | Yes |
| `RwLock<T>` | 1 value, heap | Yes | Yes (run-time) | Yes |

---

## Choosing the right one

```
Need heap allocation          → Box<T>
Need single-thread shared     → Rc<T>   (+ RefCell<T> for mut)
Need cross-thread shared      → Arc<T>  (+ Mutex<T> / RwLock<T> for mut)
Need recursive type           → Box<T>
Need trait object              → Box<dyn Trait>
```