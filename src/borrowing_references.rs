// Topic 6: Borrowing and References
// Run with: cargo run --bin borrowing_references

// References let you *borrow* a value without taking ownership.
// There are two kinds:
//   &T  — immutable reference   (&String, &i32, …)
//   &mut T — mutable reference (&mut String, &mut i32, …)

fn main() {
    // ------------------------------------------------------------------
    // &T — immutable reference
    // ------------------------------------------------------------------
    let s1 = String::from("hello");
    let len = calc_len(&s1);
    println!("'{s1}' has length {len}");

    // ------------------------------------------------------------------
    // &mut T — mutable reference
    // ------------------------------------------------------------------
    let mut s2 = String::from("hello");
    add_greeting(&mut s2);
    println!("{s2}"); // "hello, world!"

    // ------------------------------------------------------------------
    // Dangling references — Rust prevents them at compile time
    // ------------------------------------------------------------------
    // let r;
    // {
    //     let x = 5;
    //     r = &x;          // x is dropped here → r would dangle
    // }
    // println!("{r}");     // ❌ compile error: x does not live long enough

    // ==================================================================
    // Rules at a glance
    // ==================================================================
    // 1. You may have ANY NUMBER of immutable references, OR
    // 2. You may have EXACTLY ONE mutable reference.
    // 3. References must always be valid (no dangling refs).

    // ----------- Multiple immutable borrows -----------
    let t = String::from("rust");
    let r1 = &t;
    let r2 = &t;
    println!("{r1} {r2}"); // ok: multiple &T

    // ----------- NLL (Non-Lexical Lifetimes) -----------
    // A mutable borrow's lifetime ends at its last use, not at the `}`:
    let mut m = String::from("abc");
    let r = &m;
    println!("{r}");   // r's last use → borrow ends here
    m.push_str("!");   // ok! r is no longer "alive"
    println!("{m}");

    // ----------- Slices (reference into a collection) -----------
    let phrase = String::from("hello world");
    let hello = &phrase[0..5];   // &str slice
    let world = &phrase[6..11];
    println!("{hello} {world}");

    let whole = &phrase[..];     // entire string
    println!("whole = {whole}");
}
// =========================================================================
// Function signatures
// =========================================================================

fn calc_len(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it does NOT have ownership, nothing happens.
 // It doesn't drop what it references.

fn add_greeting(s: &mut String) {
    s.push_str(", world!");
}