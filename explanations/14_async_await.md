# 14 — Async / Await

## Overview

Asynchronous Rust lets you **wait** for many I/O operations in parallel without blocking OS threads. It uses **zero-cost futures**: every `.await` point compiles down to a state-machine — no heap allocation, no runtime required by the language.

```rust
async fn fetch() -> String {
    // do async I/O …
    "data".into()
}

let fut = fetch();    // returned immediately (lazy)
let result = fut.await; // suspends until ready
```

---

## The Future trait

All async code compiles to a type implementing `std::future::Future<Output = T>`:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

```
Poll::Pending  —  not ready yet; waker registered
Poll::Ready(v) —  done; v is the result
```

You rarely call `poll()` directly — async runtimes (Tokio, async-std) handle it.

### Lazy execution

A future does **nothing** until `.await`ed by a runtime:

```rust
let f = slow_operation(); // creates future — NO work done yet
// … do other work …
let r = f.await;          // first time .poll() is called
```

---

## Waker — wake notification

When a future is `Pending`, it provides a **`Waker`** to the runtime. The runtime schedules the future to be polled again when:

- A socket is readable / writable.
- A timer fires.
- A channel receives a message.

```rust
poll → Pending → register waker → timer fires → waker.wake() → poll → Ready
```

Only the **runtime** should create and manage wakers — library code just calls `cx.waker().wake_by_ref()`.

---

## async fn

Same semantics as `fn`, but returns an **anonymous `impl Future`**:

```rust
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut s = String::new();
    file.read_to_string(&mut s).await?;
    Ok(s)
}
```

Every `.await` in `async fn` is an operator — it must be called on a `Future` and only inside an async context.

### async closures

Stable since Rust 1.73 — use `async move || { … }` inside a runtime:

```rust
let fut = async move || {
    tokio::time::sleep(Duration::from_millis(10)).await;
    "done"
};
```

---

## Tokio — the de facto async runtime

`#[tokio::main]` sets up the runtime and calls `.await` on the returned future:

```rust
#[tokio::main]
async fn main() {
    read_file("data.txt").await.unwrap();
}
```

### Tokio tasks — cheap green threads

`tokio::spawn` schedules a **future** on the async runtime (not an OS thread):

```rust
tokio::spawn(async { do_work().await });
```

Tasks are:
- **Cheap** — ~64 bytes in memory.
- **M:N** — many tasks multiplexed over few OS threads.
- **Cooperative** — a `tokio::spawn` task does not run unless it `.await`s.

---

## Streams — async iterators

```rust
use tokio_stream::StreamExt;

let mut s = stream::iter(vec![1, 2, 3]);
while let Some(n) = s.next().await {
    println!("{n}");
}
```

Rust has **no** built-in `async` block syntax, but the standard library has

---

## Select — multiplex futures

```rust
use tokio::select;

let mut tick = interval(Duration::from_millis(100));
select! {
    biased; // always check timeout branch first
    _ = tick.tick() => println!("tick"),
    _ = timeout(Duration::from_millis(350), do_work()) => println!("done"),
}
```

Only the selected branch executes each round. `biased` peels one branch first to avoid starvation in simple cases.

---

## Cancellation — .drop() cancels a future

Dropping a future **cancels** it. No "finally" blocks work automatically:

```rust
let fut = do_work(); // future created
drop(fut);            // immediately cancelled — poll() never called
```

Use `futures::TryFutureExt::cancel_safe()` or `tokio::select!` with a cancellation token for Graceful shutdown.

---

## Comparison: sync vs async

| Aspect | `std::thread` | `tokio::spawn` |
|---|---|---|
| OS thread | Yes | No (M:N) |
| Scheduling | OS pre-emptive | Cooperative |
| Stack size | ~8 MiB | ~2 KiB |
| Context-switch cost | ~µs | ~ns |
| Ideal for | CPU-bound | IO-bound |

**Rule of thumb**: use `tokio::spawn` for I/O, `std::thread` / `spawn_blocking` for CPU-heavy work.
