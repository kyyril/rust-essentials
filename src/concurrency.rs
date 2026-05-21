// Topic 12: Concurrency
// Run with: cargo run --bin concurrency

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// =========================================================================
// 1. Thread creation
// =========================================================================
fn spawn_demo() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  thread says {i}");
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("  main says {i}");
        thread::sleep(Duration::from_millis(50));
    }
    handle.join().unwrap(); // wait
}

// =========================================================================
// 2. Message passing — mpsc channel
// =========================================================================
fn channel_demo(mut rx: mpsc::Receiver<String>) {
    while let Ok(msg) = rx.recv() {
        println!("  received: {msg}");
    }
}

fn channel_setup() -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        for msg in ["hello", "from", "thread"] {
            tx.send(msg.to_string()).unwrap();
        }
    });
    rx
}

// =========================================================================
// 3. Shared state — Arc<Mutex<T>>
// =========================================================================
fn shared_state_demo() {
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    for _ in 0..6 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut n = c.lock().unwrap();
                *n += 1;
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("  final = {}", *counter.lock().unwrap()); // 6000
}

// =========================================================================
// 4. Scoped threads — borrow stack data in a child thread
// =========================================================================
fn scoped_demo() {
    let data = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("  scope thread: {:?}", data); // borrow from parent stack
        });
    }); // threads joined here automatically
    println!("  scoped threads done");
}

// =========================================================================
// 5. Send / Sync — the auto-traits
// =========================================================================
fn send_sync_demo() {
    // Copy  T : Send + Sync → may cross thread boundaries
    // Arc<T> : requires T: Send + Sync
    // Rc<T>  : NOT Send / Sync — cannot cross thread boundaries
    println!("  Arc<i32>: Send is checked at compile time");
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    println!("=== 1. Thread spawn ===");
    spawn_demo();

    println!("\n=== 2. mpsc channel ===");
    let rx = channel_setup();
    channel_demo(rx);

    println!("\n=== 3. Arc<Mutex<T>> shared counter ===");
    shared_state_demo();

    println!("\n=== 4. Scoped threads ===");
    scoped_demo();

    println!("\n=== 5. Send/Sync ===");
    send_sync_demo();

    println!("\nDone.");
}