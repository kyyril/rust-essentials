# Exercise 14 — Async / Await

## Task 14.1 — async fn basics

```rust
// TODO: Write an async fn fetch(url: &str) → Result<String, io::Error>
// that does:
//   1. tokio::time::sleep(Duration::from_millis(10)).await  — mock delay
//   2. returns Ok("data".to_string())

#[tokio::main]
async fn main() {
    let body = fetch("https://example.com").await.unwrap();
    println!("fetched: {body}");
}
```

---

## Task 14.2 — concurrent tasks

```rust
// TODO: replace these 3 sequential .await calls with tokio::spawn + join!
// so all 3 run concurrently (should finish in ~100ms not 300ms).
macro task(n: u32) — sleeps n*100ms then returns n
Result = [10, 20, 30]
```

---

## Task 14.3 — select! race

```rust
use tokio::select;

#[tokio::main]
async fn main() {
    // TODO: Write a select! that:
    //   · waits 500ms on a timeout branch
    //   · receives from an mpsc channel in the other branch
    //   · whichever finishes first prints "winner"
}
```

---

## Task 14.4 — Stream + collect

```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut s = tokio_stream::iter(1..=5)
        // TODO: .then(make async closure) so each item is doubled after 50ms delay
        // .collect::<Vec<_>>()
        // .await;
}
// expect: [2,4,6,8,10]
```

---

## Task 14.5 — Build a Future manually

Write a `Countdown` future that resolves `{n, n-1, …, 1}` on each `.await` call, then returns `"blast off!"`.

```rust
struct Countdown { /* TODO */ }
impl Future for Countdown { /* TODO */ }

#[tokio::main]
async fn main() {
    let mut cd = Countdown { remaining: 3 };
    while let Poll::Pending = Pin::new(&mut cd).poll(&mut cx) { /* loop … */ }
}
```

---

## Answers

<details>
<summary>Click to reveal</summary>

```rust
// 14.1
async fn fetch(_url: &str) -> io::Result<String> {
    tokio::time::sleep(Duration::from_millis(10)).await;
    Ok("data".into())
}

// 14.2 — use join!
let (a, b, c) = tokio::join!(task(100), task(200), task(300));
assert_eq!(vec![a, b, c], vec![100, 200, 300]);

// 14.3
let (tx, mut rx) = mpsc::channel(1);
tokio::spawn(async move { tx.send("hello").await.ok(); });
select! {
    biased;
    v = rx.recv() => println!("channel: {:?}", v),
    _ = timeout(Duration::from_millis(500), async {}) => println!("timeout"),
}

// 14.4
let v: Vec<_> = tokio_stream::iter(1..=5)
    .then(|n| async move { tokio::time::sleep(50ms).await; n * 2 })
    .collect()
    .await;

// 14.5 — manual future
impl Future for Countdown { type Output = &'static str;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.remaining {
            0 => Poll::Ready("blast off!"),
            n => { self.get_mut().remaining = n - 1; Poll::Pending }
        }
    }
}
```
</details>
