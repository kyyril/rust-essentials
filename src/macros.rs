// Topic 13: Macros
// Run with: cargo run --bin macros

// =========================================================================
// Part A — Declarative macros (macro_rules!)
// =========================================================================

// --- vec! macro reimplemented ---
macro_rules! my_vec {
    // single value, repeated N times
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };

    // variadic list
    ($($x:expr),+ $(,)?) => {
        vec![$($x),+]
    };
}

// --- custom assert_eq extended with custom messages ---
macro_rules! checked_assert_eq {
    ($a:expr, $b:expr $(,)?) => {
        assert_eq!($a, $b, "assertion failed: left={}, right={}", $a, $b)
    };
}

// --- custom iterator macro ---
macro_rules! for_each {
    ($vec:ident => $x:ident => $body:expr) => {
        for $x in $vec.iter() {
            $body
        }
    };
}

// --- DSL example: key-value pairs ---
macro_rules! kv {
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut m = std::collections::HashMap::new();
            $(m.insert($key, $value.to_string());)+
            m
        }
    };
}

// =========================================================================
// Part B — Procedural macros intro (library must be proc-macro = true)
// =========================================================================

// Derive macro — the mechanism behind #[derive(Debug)]
use std::fmt;

// Manual #[derive] for our simple struct (simulating what a procedural macro generates)
#[derive(Debug)]
struct SimpleStruct {
    name: String,
    count: u32,
}

// Function-like procedural macro simulation using helpers
fn adder_proc(expr: &str) -> String {
    // In a real procedural macro this runs at compile time via syn/quote
    format!("[proc-macro-result: {}", expr)
}

// =========================================================================
// Part C — Attribute macros (conceptual / minimal working example)
// =========================================================================

// In a separate proc-macro crate you'd write:
// ```
// #[proc_macro_attribute]
// pub fn log_calls(args: TokenStream, input: TokenStream) -> TokenStream {
//     // wraps function body with println!("Entering fn …")
// }
// ```

// Workaround to demonstrate attribute-macro behaviour in a single file:
macro_rules! log_call {
    ($name:ident($($arg:ident: $ty:ty),*) $body:block) => {
        fn $name($( $arg: $ty ),*) -> String {
            println!("[log] entering {}", stringify!($name));
            $body
        }
    };
}

// --- using the log_call macro ---
log_call! {
    greet(name: &str, age: u32) {
        format!("Hello, {name}! You are {age} years old.")
    }
}

fn use_greet() {
    let msg = greet("Rustacean", 30);
    println!("{msg}");
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    // --- my_vec! ---
    let v1 = my_vec![1, 2, 3, 4];
    let v2 = my_vec![0; 5];
    println!("v1 = {:?}", v1);      // [1,2,3,4]
    println!("v2 = {:?}", v2);      // [0,0,0,0,0]

    // --- checked_assert_eq! ---
    checked_assert_eq!(2 + 2, 4);

    // --- for_each! ---
    let nums = vec![10, 20, 30];
    let mut doubled = vec![];
    for_each!(nums => n => { doubled.push(n * 2); });
    println!("doubled = {:?}", doubled); // [20,40,60]

    // --- kv! DSL ---
    let map = kv! {
        "name" => "Rust",
        "year" => 2026,
    };
    println!("kv map = {:?}", map);

    // --- procedural macro simulation ---
    println!("{}", adder_proc("x + y"));

    // --- attribute/method macro simulation ---
    use_greet();

    // --- debug print of struct (derive macro) ---
    let s = SimpleStruct { name: "hello".into(), count: 3 };
    println!("{:?}", s);
}