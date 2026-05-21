// Topic 9: Error Handling
// Run with: cargo run --bin error_handling

// Rust groups recoverable errors into two categories:
//   Option<T> — value may be absent (None)
//   Result<T, E> — operation may fail (Ok / Err)

use std::fs::File;
use std::io::{self, Read};

// =========================================================================
// 1. Unrecoverable errors — panic!
// =========================================================================
fn panic_demo() {
    // panic!("crash and burn"); // uncomment to see the panic
    vec![1, 2, 3][99];          // index out of bounds → panic
}

// =========================================================================
// 2. Recoverable with Option
// =========================================================================
fn divide_option(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// =========================================================================
// 3. Recoverable with Result
// =========================================================================
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse::<i32>()
}

// =========================================================================
// 4. Propagating errors — ? operator
// =========================================================================
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // returns Err early if file missing
    let mut s = String::new();
    f.read_to_string(&mut s)?;            // propagate read error
    Ok(s)
}

// Shortcut using fs::read_to_string
fn read_username_short() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

// =========================================================================
// 5. Custom error types
// =========================================================================
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    EmptyInput,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e)   => write!(f, "IO error: {e}"),
            AppError::Parse(e) => write!(f, "Parse error: {e}"),
            AppError::EmptyInput => write!(f, "input was empty"),
        }
    }
}

impl std::error::Error for AppError {}

// =========================================================================
// 6. unwrap / expect — quick panic on failure (dev / prototyping)
// =========================================================================
fn unwrap_demo() {
    // let val = None.expect("value must be Some"); // panics
    let val = Some(42).expect("must be Some");
    println!("unwrapped: {val}");

    let val2 = Some(10).unwrap(); // panics if None (no message)
    println!("unwrapped2: {val2}");
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    // ----- Option -----
    match divide_option(10.0, 2.0) {
        Some(v) => println!("10/2 = {v}"),
        None => println!("Cannot divide by zero"),
    }
    match divide_option(10.0, 0.0) {
        Some(v) => println!("10/0 = {v}"),
        None => println!("Cannot divide by zero"),
    }

    // ----- Option combinators -----
    let v = Some("42").and_then(|s| s.parse::<i32>().ok());
    println!("parsed: {:?}", v); // Some(42)

    // ----- Result -----
    match parse_int(" 42 ") {
        Ok(n)  => println!("parsed int = {n}"),
        Err(e) => eprintln!("parse error: {e}"),
    }

    let err = parse_int("not_a_number");
    println!("err result = {:?}", err); // Err(ParseIntError)

    // ----- if let / match on Result -----
    if let Ok(n) = parse_int("100") {
        println!("got {n} from if-let");
    }

    // ----- unwrap_demo -----
    unwrap_demo();
}