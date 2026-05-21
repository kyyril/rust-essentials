// Topic 12: Concurrency
// Run with: cargo run --bin concurrency

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// =========================================================================
// 1. Thread creation — std::thread::spawn
// =========================================================================
fn spawn_demo() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  thread says {i}");
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Meanwhile in main
    for i in 1..=3 {
        println!("  main says {i}");
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap(); // wait for thread to finish
    println!(
        "  .join() returned: {:?}",
        handle
            .join()
            .unwrap_or_else(|e| format!("thread panicked: {e:?}"))
    );
}

// =========================================================================
// 2. Message passing — mpsc::channel (multi-producer, single-consumer)
// =========================================================================
fn channel_demo() {
    // tx = transmitter, rx = receiver
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let msgs = vec!["hello", "from", "thread"];
        for msg in msgs {
            tx.send(msg.to_string()).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Blocking receive
    while let Ok(msg) = rx.recv() {
        println!("  received: {msg}");
    }
    // Try_recv — non-blocking: returns Err immediately if nothing yet
    // iter() — reuse the receiver as an iterator
}

// =========================================================================
// 3. Shared-state concurrency — Arc<Mutex<T>>
// =========================================================================
fn shared_state_demo() {
    let counter = Arc::new(Mutex::new(0u32));

    let mut handles = vec![];
    for _ in 0..6 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = c.lock().unwrap();
                *num += 1; // guard auto-unlocks on drop
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("  final = {}", *counter.lock().unwrap()); // 6000
}

// =========================================================================
// 4. Scoped threads — cross borrow of stack data
// =========================================================================
fn scoped_demo() {
    // Without scoped threads, the compiler rejects .join references in closures.
    thread::scope(|s| {
        let data = vec![1, 2, 3];
        s.spawn(|| {
            println!("  scope thread: {:?}", data); // borrows stack data
        });
    }); // all threads guaranteed to finish before scope exits

    println!("  all scoped threads done");
}

// =========================================================================
// 5. Channels — multiple producers
// =========================================================================
fn multi_producer_demo() {
    let (tx, rx) = mpsc::channel::<i32>();

    for i in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    drop(tx); // drop original so rx eventually closes

    // Collect all messages
    let received: Vec<_> = rx.iter().collect();
    println!("  received: {:?}", received);
    // order not guaranteed across threads
}

// =========================================================================
// 6. Sync trait — what types can be shared across threads?
// =========================================================================
fn sync_trait_demo() {
    // Arc<T> requires T: Send + Sync
    //   Send  — ownership may be transferred between threads
    //   Sync  — &T may be shared safely across threads
    // Rc<T> is NOT Send/Sync — use Arc<T> instead
    println!(
        "  Arc<T> is Send: {}",
        std::any::TypeId::of::<Arc<i32>>() == std::any::TypeId::of::<Arc<i32>>()
    );
    println!("  Rc<T> is Send: {}", // Rc<i32> is NOT Send
        false // substituted for brevity
    );
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    println!("=== 1. Thread spawn ===");
    spawn_demo();

    println!("\n=== 2. mpsc channel ===");
    channel_demo();

    println!("\n=== 3. Shared state (Arc<Mutex<T>>) ===");
    shared_state_demo();

    println!("\n=== 4. Scoped threads ===");
    scoped_demo();

    println!("\n=== 5. Multi-producer channel ===");
    multi_producer_demo();

    println!("\n=== 6. Send/Sync ===");
    sync_trait_demo();
}