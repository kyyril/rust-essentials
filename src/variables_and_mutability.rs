// Topic 1: Variables and Mutability
// Run with: cargo run --bin variables_and_mutability

fn main() {
    // ------------------------------------------------------------------
    // 1. Immutability (default)
    // ------------------------------------------------------------------
    let x = 5;
    println!("x = {x}");

    // Uncommenting the line below causes a compile error:
    // x = 6;  // error: cannot assign twice to immutable variable

    // ------------------------------------------------------------------
    // 2. Mutable variables
    // ------------------------------------------------------------------
    let mut y = 10;
    println!("y (initial) = {y}");
    y = 20;
    println!("y (updated)  = {y}");

    // ------------------------------------------------------------------
    // 3. Shadowing
    //    The new variable "shadows" the old one inside this scope.
    // ------------------------------------------------------------------
    let z = 42;
    let z = z + 1;   // still immutable, but a new binding
    println!("z = {z}");

    // Shadowing also lets you change the *type*
    let spaces = "   ";
    let spaces = spaces.len(); // now it's an integer
    println!("spaces = {spaces}");

    // ------------------------------------------------------------------
    // 4. Constants
    //    - always immutable
    //    - naming convention: SCREAMING_SNAKE_CASE
    //    - type must be annotated
    // ------------------------------------------------------------------
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");
}