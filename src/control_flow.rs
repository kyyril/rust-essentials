// Topic 4: Control Flow
// Run with: cargo run --bin control_flow

fn main() {
    // ==================================================================
    // IF — expressions (no parens required around condition)
    // ==================================================================
    let num = 6;

    let msg = if num % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("{num} is {msg}");

    // Multiple branches
    let num2 = 3;
    let result = if num2 < 0 {
        "negative"
    } else if num2 == 0 {
        "zero"
    } else if num2 < 10 {
        "small positive"
    } else {
        "large positive"
    };
    println!("num2 is {result}");

    // ==================================================================
    // Loop — infinite, labelled, returning values
    // ==================================================================

    // loop returns a value (with break <value>)
    let mut count = 0;
    let loop_result = loop {
        count += 1;
        if count == 5 {
            break count * 2;
        }
    };
    println!("loop_result = {loop_result}"); // 10

    // labelled loops (break outer from nested)
    'outer: for item in 1..3 {
        'inner: for j in 1..4 {
            if j == 2 {
                break 'outer; // exits the outer loop
            }
            println!("  item={item} inner={j}");
        }
    }

    // ==================================================================
    // While — condition evaluated every iteration
    // ==================================================================
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("LIFTOFF");

    // ==================================================================
    // For — ranges and iterators (preferred over while/loop for this)
    // ==================================================================
    println!("Range (1..4):");
    for i in 1..4 {
        println!("  {i}");
    }

    // iter()  — borrow each element
    // into_iter() — take ownership
    // iter_mut() — mutable borrow

    println!("Array iteration:");
    let pets = ["cat", "dog", "hamster"];
    for pet in pets.iter() {
        println!("  pet: {pet}");
    }

    // ==================================================================
    // Match — exhaustive pattern matching
    // ==================================================================
    let coin = 2;
    let c = match coin {
        1 => "Penny",
        2 => "Nickel",
        5 => "Dime",
        10 => "Quarter",
        _ => "Unknown coin", // _ is the catch-all
    };
    println!("coin = {c}");

    // Match on booleans
    let status = match (6 % 2 == 0) {
        true => "even match",
        false => "odd match",
    };
    println!("{status}");
}