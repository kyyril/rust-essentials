# Exercise 8 — Enums and Pattern Matching

## Task 8.1 — Add a new enum variant

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        // TODO: add the other 3 variants
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter)); // expect 25
}
```

---

## Task 8.2 — Data in enum variants

```rust
enum Action {
    Move { x: i32, y: i32 },
    Speak(String),
    Wait,
}

// TODO: write a fn perform(a: Action) -> String that returns:
//   Move  → "Moved to (x, y)"
//   Speak → "Said: <text>"
//   Wait  → "Waiting…"
fn perform(a: Action) -> String {
    // ── TODO ── use match ──
}
```

---

## Task 8.3 — Match guards

```rust
fn describe(n: i32) -> &'static str {
    // TODO: return
    //   "zero or positive even" when n >= 0 & even
    //   "zero or positive odd"  when n >= 0 & odd
    //   "negative even"          when n < 0  & even
    //   "negative odd"           when n < 0  & odd
    //   use a single match expression
}
```

---

## Task 8.4 — Option::map chain

```rust
fn main() {
    let v1: Option<&str> = Some("42");
    // TODO: parse v1 as i32, multiply by 2, and print with format!("{}", result)
    // Hint: chain .and_then() and .map()
    //          or `.map(|n| n  * 2) and flatten.
    let r = v1.___.___(|s: &str| {
        s.parse::<i32>().ok()
    });
    println!("{:?}", r); // hope: Some(84)
}
```

---

## Task 8.5 — if let vs match

For each case, decide and write the better construct:

```rust
// Case A: only care about Some(x) from an Option, ignore None
//         → if let

// Case B: need to handle Some, None, AND an Err in a Result
//         → match

// Case C: inspect a custom enum with exactly 4 variants exhaustively
//         → match
```

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 8.1
fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 8.2
fn perform(a: Action) -> String {
    match a {
        Action::Move { x, y } => format!("Moved to ({},{})", x, y),
        Action::Speak(txt)   => format!("Said: {}", txt),
        Action::Wait         => "Waiting…".to_string(),
    }
}

// 8.3
fn describe(n: i32) -> &'static str {
    match n {
        x if x >= 0 && x % 2 == 0 => "zero or positive even",
        x if x >= 0               => "zero or positive odd",
        x if x % 2 == 0           => "negative even",
        _                         => "negative odd",
    }
}

// 8.4
let r = v1.and_then(|s| s.parse::<i32>().ok()).map(|n| n * 2);
```
</details>
