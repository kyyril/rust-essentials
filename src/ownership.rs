// Topic 5: Ownership
// Run with: cargo run --bin ownership

// Ownership = 3 core rules:
//   1. Each value has exactly ONE owner.
//   2. When the owner goes out of scope, the value is dropped.
//   3. There can be only ONE owner at a time.

fn main() {
    // ------------------------------------------------------------------
    // Scope & drop
    // ------------------------------------------------------------------
    {
        let s = String::from("hello"); // s is valid from here
        println!("{s}");               // use s
    } // s goes out of scope → memory freed

    // ------------------------------------------------------------------
    // Move semantics
    //    s1 is MOVED into s2; s1 is no longer valid.
    // ------------------------------------------------------------------
    let s1 = String::from("hello");
    let s2 = s1;              // s1 is MOVED to s2
    // println!("{s1}");       // ❌ compile error: s1 was moved
    println!("{s2}");           // ok

    // ------------------------------------------------------------------
    // Clone — make a deep copy when you NEED both copies
    // ------------------------------------------------------------------
    let s3 = String::from("world");
    let s4 = s3.clone();        // heap data is actually duplicated
    println!("s3={s3} s4={s4}"); // both are valid

    // ------------------------------------------------------------------
    // Stack-only data (Copy) — cheap bitwise copy
    // ------------------------------------------------------------------
    let x = 5;
    let y = x;          // copy (no move) because i32 is Copy
    println!("x={x} y={y}"); // both fine

    // ==================================================================
    // Functions and ownership
    // ==================================================================
    let s = String::from("hello");
    takes_ownership(s);           // s moved into function
    // println!("{s}");           // ❌ s is no longer valid

    let n = 5;
    makes_copy(n);                // i32 implements Copy
    println!("{n}");              // still valid

    // ==================================================================
    // Returning ownership
    // ==================================================================
    let s2 = gives_ownership();    // moved here
    println!("{s2}");

    let s3 = String::from("hello"); // s3 enters scope
    let s4 = takes_and_gives_back(s3); // s3 moved, returned as s4
    println!("{s4}");

    // ==================================================================
    // WHY ownership?  → memory safety without GC.
    //   - No double-free
    //   - No dangling references
    //   - No use-after-free
    // ==================================================================
}

fn takes_ownership(s: String) {
    println!("{s}");
} // s dropped here

fn makes_copy(x: i32) {
    println!("{x}");
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s // moved out
}

fn takes_and_gives_back(s: String) -> String {
    s // same value returned
}