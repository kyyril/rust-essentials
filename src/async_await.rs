// Topic 14: Async / Await
// Run with: cargo add tokio --features rt,rt-multi-thread,macros
//        cargo run --bin async_await

// This file uses the standard library executor for simple examples
// and the async-std or tokio runtime for the full playground.
//
// Run: cargo add tokio --features rt,rt-multi-thread,macros
// then: cargo run --bin async_await

// =========================================================================
// 1. async fn — returns a Future, not a value
// =========================================================================
async fn slow_add(a: i32, b: i32) -> i32 {
    // mock async delay
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    a + b
}

// =========================================================================
// 2. .await — yields control until the future is ready
// =========================================================================
async fn await_demo() {
    let r = slow_add(3, 4).await;
    println!("  3 + 4 = {r}");
}

// =========================================================================
// 3. spawn — runs a future on the runtime concurrently
// =========================================================================
async fn spawn_demo() {
    let h1 = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("  task 1 done");
    });
    let h2 = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
        println!("  task 2 done");
    });
    let _ = tokio::join!(h1, h2); // wait for both
}

// =========================================================================
// 4. Channels — mpsc in async world
// =========================================================================
use tokio::sync::mpsc;

async fn channel_demo() {
    let (tx, mut rx) = mpsc::channel(4);

    tokio::spawn(async move {
        for i in 0..3 {
            tx.send(format!("msg-{i}")).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("  got {msg}");
    }
}

// =========================================================================
// 5. Shared state — Mutex / RwLock in async context
// =========================================================================
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

async fn async_shared_demo() {
    let counter = Arc::new(TokioMutex::new(0usize));

    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..500 {
                let mut n = c.lock().await;
                *n += 1;
            }
        }));
    }
    for h in handles { h.await.unwrap(); }

    println!("  async counter = {}", *counter.lock().await); // 2000
}

// =========================================================================
// 6. select! — race multiple futures
// =========================================================================
use tokio::time::{interval, timeout, Duration};

async fn select_demo() {
    use tokio::select;

    let mut ticker = interval(Duration::from_millis(100));
    let delay = timeout(Duration::from_millis(350), async {
        loop {
            ticker.tick().await;
            println!("  tick");
        }
    });

    select! {
        biased;
        _ = delay => println!("  timeout — done"),
    }
}

// =========================================================================
// 7. Future trait — building a simple future manually
// =========================================================================
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// A poll-based Future that resolves after a fixed number of polls.
struct CounterFuture {
    count:   u8,
    max:     u8,
}

impl Future for CounterFuture {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        if self.count >= self.max {
            Poll::Ready(self.count)
        } else {
            Poll::Pending
        }
    }
}

// =========================================================================
// 8. Block on a future without a runtime (manual executor)
// =========================================================================
fn block_on<F: Future>(mut fut: F) -> F::Output
where
    F::Output: Unpin,
    F: 'static,
{
    use std::mem::ManuallyDrop;

    let mut fut = unsafe { ManuallyDrop::new(Pin::new_unchecked(&mut fut)) };
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => {
                std::thread::yield_now();
            }
        }
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    const VTABLE: &RawWakerVTable =
        &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), VTABLE)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

// =========================================================================
// Main — tokio runtime entry point
// =========================================================================
#[tokio::main]
async fn main() {
    println!("=== 1. async fn + .await ===");
    await_demo().await;

    println!("\n=== 2. tokio::spawn ===");
    spawn_demo().await;

    println!("\n=== 3. async mpsc channel ===");
    channel_demo().await;

    println!("\n=== 4. Arc<Mutex> in async ===");
    async_shared_demo().await;

    println!("\n=== 5. select! ===");
    select_demo().await;

    println!("\n=== 6. Manual Future + block_on ===");
    let val = block_on(CounterFuture { count: 0, max: 5 });
    println!("  CounterFuture resolved to {}", val);

    println!("\nAll async demos complete.");
}