// OOP in Rust
// Run with: cargo run --bin oop

// =========================================================================
// 1. Traits — define shared behaviour (like Java interfaces / Go interfaces)
// =========================================================================
/// Common behaviour for "something drawable".
trait Drawable {
    /// Draw this item at (x, y).
    fn draw(&self, x: u32, y: u32);

    /// Compute and return the area.  Default implementation — override if better.
    fn area(&self) -> u32;
}

// =========================================================================
// 2. Structs — the "objects"
// =========================================================================
struct Circle {
    radius: u32,
    color:  &'static str,
}

struct Rectangle {
    width:  u32,
    height: u32,
    color:  &'static str,
}

// =========================================================================
// 3. Implement traits for structs  (substituting "inheritance")
// =========================================================================
impl Drawable for Circle {
    fn draw(&self, x: u32, y: u32) {
        println!(
            "Drawing a {} circle (r={}) at ({}, {})",
            self.color, self.radius, x, y
        );
    }

    fn area(&self) -> u32 {
        // π · r²  (using 3 for simplicity)
        3 * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self, x: u32, y: u32) {
        println!(
            "Drawing a {} rectangle {w}×{h} at ({x},{y})",
            self.color, w = self.width, h = self.height
        );
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// =========================================================================
// 4. Function polymorphism through TRAIT BOUNDS (static dispatch)
// =========================================================================
fn print_area<T: Drawable>(item: &T) {
    println!("Area = {}", item.area());
}

// =========================================================================
// 5. Trait objects — DYNAMIC DISPATCH (run-time polymorphism)
//    Any type that implements Drawable can be stored in &dyn Drawable
// =========================================================================
fn draw_scene(items: &[&dyn Drawable]) {
    for (i, item) in items.iter().enumerate() {
        item.draw(i as u32 * 20, 0);
        println!("  → area={}", item.area());
    }
}

// =========================================================================
// 6. Encapsulation — public vs private fields
//    Everything is *private by default*; expose only what is needed via pub.
// =========================================================================
pub struct BankAccount {
    pub owner:    &'static str, // public — caller can read
    balance:     u64,           // private — no direct external access
}

impl BankAccount {
    /// Constructor — associated function (no self)
    pub fn new(owner: &'static str, initial: u64) -> Self {
        Self { owner, balance: initial }
    }

    /// Read-only accessor
    pub fn balance(&self) -> u64 {
        self.balance          // OK: accessing own private field
    }

    /// Mutating method — encapsulates validation
    pub fn deposit(&mut self, amount: u64) {
        if amount > 0 {
            self.balance += amount;
            println!("Deposited {} → new balance: {}", amount, self.balance);
        }
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            println!("Insufficient funds ({})", self.balance);
            false
        } else {
            self.balance -= amount;
            println!("Withdrew {} → new balance: {}", amount, self.balance);
            true
        }
    }
}

// =========================================================================
// 7. Composition over inheritance
//    Rust has no class inheritance.  Share behaviour by embedding structs.
// =========================================================================
struct Engine {
    horsepower: u32,
}

impl Engine {
    fn start(&self) {
        println!("Engine started — {} hp", self.horsepower);
    }
}

struct Car {
    make:  &'static str,
    model: &'static str,
    engine: Engine,          // Car "has-an" Engine (composition)
}

impl Car {
    fn new(make: &'static str, model: &'static str, hp: u32) -> Self {
        Self { make, model, engine: Engine { horsepower: hp } }
    }

    fn start(&self) {
        // Delegate to the composed Engine
        self.engine.start();
        println!("{} {} is ready to go!", self.make, self.model);
    }
}

// =========================================================================
// 8. Polymorphic method — same method name on types that share a trait
// =========================================================================
trait Flyable {
    fn fly(&self);
}

impl Flyable for Circle {
    // Circles don't really fly — intentionally silly
    fn fly(&self) {
        println!("The {} circle floats gently through the air.", self.color);
    }
}

impl Flyable for Car {
    fn fly(&self) {
        println!("The {} {} zooms through the sky!", self.make, self.model);
    }
}

fn make_them_fly<T: Flyable>(thing: &T) {
    thing.fly();
}

// =========================================================================
// 9. Default trait implementations (like default interface methods)
// =========================================================================
trait Greet {
    fn name(&self) -> &str;

    // A default implementation — concrete types can choose to override.
    fn greet(&self) {
        println!("Hello, I'm {}", self.name());
    }
}

struct Robot {
    designation: &'static str,
}
impl Greet for Robot {
    fn name(&self) -> &str { self.designation }
}

struct Android {
    alias: String,
}
impl Greet for Android {
    fn name(&self) -> &str { &self.alias }

    fn greet(&self) {
        // Override the default — static trait methods cannot be overridden,
        // so use a normal function or a custom helper instead.
        println!("Beep boop — designation: {}", self.alias);
    }
}

// =========================================================================
// 10. Object-safe vs non-object-safe traits
// =========================================================================
/// Object-safe: can be used as a trait object
trait ObjectSafeTrait {
    fn object_safe_method(&self);
}

impl ObjectSafeTrait for Circle {
    fn object_safe_method(&self) {
        println!("Circle is object-safe! radius={}", self.radius);
    }
}

fn use_object_safe(item: &dyn ObjectSafeTrait) {
    item.object_safe_method();
}

/// NOT object-safe: has a Self: Sized return type.
/// (Traits with generic methods cannot be turned into trait objects.)
#[allow(dead_code)]
trait GenericTrait {
    fn generic_method(&self) -> Self;
}

// =========================================================================
// Main — run everything
// =========================================================================
fn main() {
    println!("=== 1. Traits and implementations ===");
    let circle = Circle { radius: 10, color: "blue" };
    let rect   = Rectangle { width: 8, height: 6, color: "green" };
    print_area(&circle);   // static dispatch via generics
    print_area(&rect);

    println!("\n=== 2. Trait objects (dynamic dispatch) ===");
    let scene: Vec<&dyn Drawable> = vec![&circle, &rect, &circle];
    draw_scene(&scene);

    println!("\n=== 3. Encapsulation ===");
    let mut acct = BankAccount::new("Alice", 1_000);
    println!("Owner: {}  Balance: {}", acct.owner, acct.balance());
    acct.deposit(500);
    acct.withdraw(300);
    acct.withdraw(2_000); // insufficient funds

    println!("\n=== 4. Composition ===");
    let car = Car::new("Toyota", "Corolla", 140);
    car.start();

    println!("\n=== 5. Shared trait method (polymorphism) ===");
    make_them_fly(&circle);
    make_them_fly(&car);

    println!("\n=== 6. Default trait implementation ===");
    let robo = Robot { designation: "RB-01" };
    robo.greet(); // uses default impl
    let droid = Android { alias: "AR-7".to_string() };
    droid.greet(); // overrides default

    println!("\n=== 7. Trait objects (object-safe) ===");
    use_object_safe(&circle);

    println!("\n=== Summary ===");
    println!("Rust OOP pillars mapped to language features:");
    println!("  Encapsulation  → pub / private fields + modules");
    println!("  Polymorphism   → traits / generics / trait objects");
    println!("  Inheritance/Reuse → composition, trait bounds, mix-in impls");
    println!("  Abstraction    → trait definitions");
}