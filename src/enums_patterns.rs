// Topic 8: Enums and Pattern Matching
// Run with: cargo run --bin enums_patterns

use std::option::Option::{self, Some, None}; // std::option::Option + variants

// =========================================================================
// Enums — a type that can be ONE OF several variants
// =========================================================================
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },   // named fields (like a struct)
    Write(String),              // single value
    ChangeColor(i32, i32, i32), // tuple-style
}

impl Message {
    // Methods on enums
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => println!("Moving to ({x},{y})"),
            Message::Write(txt) => println!("Writing: {txt}"),
            Message::ChangeColor(r, g, b) => {
                println!("Color changed to RGB({r},{g},{b})")
            }
        }
    }
}

// =========================================================================
// Pattern matching with match
// =========================================================================
fn describe_coin(c: Coin) -> &'static str {
    match c {
        Coin::Penny => "A Penny!",
        Coin::Nickel => "A Nickel",
        Coin::Dime => "A Dime",
        Coin::Quarter => "A Quarter",
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        // ==== match guard (extra condition) ====
        Coin::Quarter => 25,
        other => {
            let base = match other {
                Coin::Nickel => 5,
                Coin::Dime => 10,
                _ => 0,
            };
            base
        }
    }
}

// =========================================================================
// if let and let else — concise single-pattern matching
// =========================================================================
fn process_optional(opt: Option<i32>) {
    // if let — execute block only when pattern matches
    if let Some(x) = opt {
        println!("Got value: {x}");
    } else {
        println!("Got None");
    }
}

// let else — unwrap-or-else pattern
fn unwrap_or_default(opt: Option<i32>) -> i32 {
    let Some(value) = opt else { return 0; };
    value
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    // ----- enum with method -----
    let msg = Message::Move { x: 10, y: 20 };
    msg.call();

    let msg2 = Message::Write(String::from("hello"));
    msg2.call();

    // ----- coin enum -----
    let coin = Coin::Penny;
    println!("{}", describe_coin(coin)); // "A Penny!"
    println!("value = {}", value_in_cents(Coin::Quarter)); // 25

    // ----- Option -----
    let some_num: Option<i32> = Some(5);
    let no_num: Option<i32> = None;

    match some_num {
        Some(n) => println!("Got {n}"),
        None => println!("No number"),
    }

    match no_num {
        Some(n) => println!("Got {n}"),
        None => println!("No number"), // matched
    }

    // ----- if let -----
    process_optional(Some(42));    // Got value: 42
    process_optional(None);        // Got None

    // ----- let else -----
    println!("unwrap = {}", unwrap_or_default(Some(7))); // 7
    println!("unwrap = {}", unwrap_or_default(None));    // 0
}