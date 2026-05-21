// Topic 2: Data Types (Scalar & Compound)
// Run with: cargo run --bin data_types

fn main() {
    // ==================================================================
    // SCALAR TYPES — represent a single value
    // ==================================================================

    // ---------- Integers ----------
    // Signed:   i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    // _ (underscore separator): 1_000_000
    let a: i32 = -42;
    let b: u32 = 42;
    println!("a = {a}  b = {b}");

    // ---------- Floating-point ----------
    let f: f64 = 3.14159;
    println!("pi ≈ {f}");

    // ---------- Boolean ----------
    let t: bool = true;
    println!("true = {t}");

    // ---------- Character ----------
    // char = 4-byte Unicode scalar value
    let c: char = '🔥';
    println!("emoji = {c}");

    // ---------- Numeric operations ----------
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let rem = 43 % 5;
    println!(
        "sum={sum} diff={diff} prod={prod} quot={quot} rem={rem}"
    );

    // ==================================================================
    // COMPOUND TYPES — group multiple values
    // ==================================================================

    // ---------- Tuple ----------
    // Fixed length; may contain mixed types
    let tup: (i32, f64, char) = (500, 6.4, 'x');
    // Destructure
    let (w, _, z) = tup;
    // Index access
    println!("first={} third={}", tup.0, tup.2);

    // ---------- Array ----------
    // Fixed length; all elements same type
    let months = [
        "Jan","Feb","Mar","Apr","May","Jun",
        "Jul","Aug","Sep","Oct","Nov","Dec",
    ];
    println!("first month = {}", months[0]);
    // Explicit type declaration
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    // Repeat same value: [value; count]
    let zeros = [0u8; 3]; // [0, 0, 0]
    println!("zeros = {:?}", zeros);
}