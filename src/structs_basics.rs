// Topic 7: Structs
// Run with: cargo run --bin structs_basics

// A struct groups related data. Three common kinds:
//   - Named-field struct
//   - Tuple struct
//   - Unit struct

// =========================================================================
// Named-field struct
// =========================================================================
struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    // Constructor (convention: new)
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    // Associated function (no self param)
    fn new_inactive(username: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            active: false,
        }
    }

    // Method — borrowed self
    fn greet(&self) -> String {
        format!("Hello, {}!", self.username)
    }

    // Consuming method — takes ownership
    fn deactivate(mut self) -> Self {
        self.active = false;
        self
    }
}

// =========================================================================
// Tuple struct — useful when naming the entire tuple matters
// =========================================================================
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// =========================================================================
// Unit struct — marker types
// =========================================================================
struct AlwaysEqual;

// =========================================================================
fn main() {
    // ---------- named-field ----------
    let mut user = User::new("alice", "alice@example.com");
    println!("{} ({})", user.greet(), user.email);
    user.email = String::from("alice@newdomain.com");
    println!("updated email: {}", user.email);

    // ---------- struct update syntax ----------
    let another_user = User::new("alice", "alice@newdomain.com");
    let user2 = User {
        email: String::from("bob@example.com"),
        ..another_user // copy remaining fields from another instance
    };

    // ---------- tuple struct ----------
    let black = Color(0, 0, 0);
    println!("R={} G={} B={}", black.0, black.1, black.2);

    // ---------- unit struct ----------
    let _subject = AlwaysEqual; // cannot be constructed with ()
    println!("AlwaysEqual marker exists");

    // ---------- derive common traits ----------
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    }

    let rect = Rectangle { width: 30, height: 50 };
    println!("rect = {:?}", rect);        // debug print
    println!("area = {}", rect.area());   // 1500
}