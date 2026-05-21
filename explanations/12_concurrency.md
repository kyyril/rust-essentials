# 12 — Concurrency

## Overview

Rust's concurrency model is built around **fearless concurrency**: the type system catches race conditions, deadlocks, and data races **at compile time**.

Spoiler: there are no data races in safe Rust.

---

## OS Threads — `std::thread`

```rust
let handle = thread::spawn(|| {
    // child thread work
});

handle.join().unwrap(); // wait for thread to finish
```

`thread::spawn` returns a `JoinHandle<T>` — block until the child finishes.

### Thread-local data

Each thread has its own **stack** and **local storage**. Passing arguments by value transfers ownership to the child thread.

```rust
let msg = "hello".to_string();
thread::spawn(move || println!("{msg}")); // move captures msg
```

The `move` keyword forces the closure to take ownership of captured variables — prevents dangling borrows crossing thread boundaries.

---

## Message Passing — `mpsc::channel`

**Go proverb: "Do not communicate by sharing memory; share memory by communicating."**

Rust provides **multi-producer, single-consumer** (MPSC) channels:

```rust
let (tx, rx) = mpsc::channel::<String>();

thread::spawn(move || {
    tx.send("hello".to_string()).unwrap();
});

while let Ok(msg) = rx.recv() {
    println!("{msg}");
}
```

| Function | Behaviour |
|---|---|
| `tx.send(v)` | Push value; blocks if receiver buffer full |
| `rx.recv()` | Blocking receive |
| `rx.try_recv()` | Non-blocking; returns `TryRecvError::Empty` if no message |
| `rx.iter()` | Iterator over all remaining messages |

Drop the transmitter to signal "no more messages" — `recv()` then returns `Err`.

---

## Shared-State Concurrency — `Mutex<T>` + `Arc<T>`

When multiple threads need to mutate the **same** value:

```rust
let counter = Arc::new(Mutex::new(0));

for _ in 0..10 {
    let c = Arc::clone(&counter);
    thread::spawn(move || {
        let mut n = c.lock().unwrap();
        *n += 1;
    });
}
```

### `Mutex<T>` — mutual exclusion

```
┌── Mutex ──► acquires OS mutex on .lock()
│               ↓
│          ┌─ MutexGuard<'_, T> ── derefs to &mut T
│          └─ auto-drops → os_unlock
└── Only one thread holds the guard at once
```

`lock()` **blocks** until the lock is free. The `MutexGuard` auto-unlocks at end of scope.

### `RwLock<T>` — many readers / one writer

| Operation | Concurrency |
|---|---|
| `read()` | Multiple simultaneous readers |
| `write()` | Exclusive writer (blocks all readers) |

Use `RwLock` when reads outnumber writes — significantly cheaper per-read than `Mutex`.

---

## `Arc<T>` — cross-thread shared ownership

`Arc<T>` is the thread-safe `Rc<T>`. It uses **atomic reference counting** and implements `Send + Sync`.

```rust
let shared = Arc::new(Mutex::new(data));
let handles: Vec<_> = (0..THREADS)
    .map(|_| {
        let s = Arc::clone(&shared);
        thread::spawn(move || {
            let mut d = s.lock().unwrap();
            *d += 1;
        })
    })
    .collect();
for h in handles { h.join().unwrap(); }
```

> `Arc<T>` prevents the value from being freed while any thread still holds it.

---

## Scoped threads — `thread::scope`

Scoped threads let child threads **borrow stack data** from the parent:

```rust
let data = heavy_computation();

thread::scope(|s| {
    // data is borrowed here — no move needed
    s.spawn(|| process(&data));
    s.spawn(|| validate(&data));
}); // all threads joined before scope exits
```

Without scoped threads, `thread::spawn` requires `'static` — all captured data must outlive the thread. Scoped threads relax this guarantee.

---

## `Send` and `Sync` — the auto-traits

Every type automatically implements (or does not implement) two **marker traits**:

| Trait | Contract |
|---|---|
| `Send` | Ownership may safely transfer to another thread |
| `Sync` | `&T` may safely share across threads |

| Type | Send | Sync |
|---|---|---|
| `Arc<T>` | ✅ (if T: Send+Sync) | ✅ |
| `Mutex<T>` | ✅ | ✅ |
| `Rc<T>` | ❌ | ❌ |
| `RefCell<T>` | ❌ | ❌ |

The compiler catches violations at the point they occur — no runtime surprises.

---

## Common Patterns

| Problem | Idiomatic solution |
|---|---|
| Map / reduce in parallel | `par_iter()` from `rayon` crate |
| Job queue | `crossbeam-channel` or `tokio::sync::mpsc` |
| Broadcast to many | `broadcast` channel |
| Select on multiple channels | `crossbeam-channel::select!` / `tokio::select!` |
| Actor pattern | `actix` or `tonic` grpc |

---

## Common Pitfalls

| Mistake | Fix |
|---|---|
| Data race | Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |
| Deadlock | Hold locks in consistent order; prefer `RwLock` |
| `Rc` across threads | Replace with `Arc` |
| Lost wake-ups | Use proper Condvar / channel instead of busy-wait |
