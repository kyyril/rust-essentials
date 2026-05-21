// Topic 3: Functions
// Run with: cargo run --bin functions_basics

// ------------------------------------------------------------------
// No return type annotation = returns ()
// ------------------------------------------------------------------
fn print_info() {
    println!("Hello from print_info!");
}

// ------------------------------------------------------------------
// Parameters: state type of each param
// ------------------------------------------------------------------
fn add(a: i32, b: i32) -> i32 {
    a + b  // last expression IS the return value (no semicolon)
}

// ------------------------------------------------------------------
// Explicit early return with return keyword
// ------------------------------------------------------------------
fn early_return(flag: bool) -> i32 {
    if flag { return -1; }
    0
}

// ------------------------------------------------------------------
// Statements vs Expressions
//   statements: do work, do NOT return a value  → end with ;
//   expressions: evaluate to a value             → no trailing ;
// ------------------------------------------------------------------
fn whats_an_expression() -> i32 {
    let x = {
        let y = 10;
        y + 5   // ← this block evaluates to 15
    };
    x   // ← SOURCES 15
}

// ------------------------------------------------------------------
// Multiple return values via a tuple
// ------------------------------------------------------------------
fn divide(a: f64, b: f64) -> (f64, &'static str) {
    if b == 0.0 {
        return (0.0, "division by zero");
    }
    (a / b, "ok")
}

// ------------------------------------------------------------------
// Main
// ------------------------------------------------------------------
fn main() {
    print_info();
    println!("5 + 7 = {}", add(5, 7));
    println!("early_return(true)  = {}", early_return(true));
    println!("early_return(false) = {}", early_return(false));
    println!("whats_an_expression() = {}", whats_an_expression());

    let (result, status) = divide(10.0, 4.0);
    println!("10 / 4 = {result} [{status}]");

    let (r2, s2) = divide(10.0, 0.0);
    println!("10 / 0 = {r2} [{s2}]");
}