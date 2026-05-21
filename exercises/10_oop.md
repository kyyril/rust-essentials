# Exercise 10 — OOP in Rust

> Rust doesn't have classes. Practice the idiomatic replacements instead.

---

## Task 10.1 — Define a trait

```rust
// TODO: define a trait `Describable` with ONE method:
//    fn describe(&self) -> &str;
trait ___ {
    // ── TODO ──
}

// TODO: implement Describable for this struct
struct Cat {
    name:      &'static str,
    meow_pitch: u8,   // 1..=255
}

impl ___ for Cat {
    fn describe(&self) -> &str {
        // ── TODO ──
    }
}

fn show<T: Describable>(t: &T) {
    println!("{}", t.describe());
}

fn main() {
    let whiskers = Cat { name: "whiskers", meow_pitch: 220 };
    show(&whiskers);
    // expect: whiskers meows at 220 Hz
}
```

---

## Task 10.2 — Default implementation

```rust
// TODO: add a default method `fn introduce(&self)` inside `Describable`
//       that prints "Hi, I'm {describe()}".
// Then add `fn introduce<'a>(&self)` to Cat override the default.

fn main() {
    whiskers.introduce();
    // expect: Hi, I'm whiskers meows at 220 Hz
}
```

---

## Task 10.3 — Encapsulation

```rust
// TODO: make `balance` private in SavingsAccount below.
// Expose it ONLY through a getter and a deposit method.
// The raw field must NOT be accessible from `main`.

pub struct ___ {
    owner:    &'static str,
    balance:  u64,   // ← private
}

impl ___ {
    // ── TODO ──
}

fn main() {
    let mut acc = SavingsAccount::new("Bob", 500);
    println!("{}", acc.balance());   // 500
    acc.deposit(250);
    println!("{}", acc.balance());   // 750
    // println!("{}", acc.balance);  // ← this must NOT compile
}
```

---

## Task 10.4 — Composition

```rust
// TODO: a `Phone` "has-a" `Battery`.
// - struct Battery { capacity_mah: u32 }
// - struct Phone      { model: &'static str, battery: Battery }
// - impl Phone {
//       fn new(model, capacity_mah) → Self
//       fn talk(&self, minutes: u32)  → Battery drains by minutes * 0.5 %
//       fn charge(&mut self)           → restores to 100 %
//     }
// - impl Battery must expose fn remaining(&self) -> u32 (0..=capacity_mah)

fn main() {
    let mut p = Phone::new("Pixel 9", 4600);
    p.talk(60);
    println!("{} mAh left", p.battery.remaining());
    p.charge();
    println!("{} mAh after charge", p.battery.remaining());
}
```

---

## Task 10.5 — Trait object (dynamic dispatch)

```rust
trait Animal {
    fn speak(&self);
}

struct Dog { name: &'static str }
struct Bird { name: &'static str }

// TODO: implement Animal for each struct.
// TODO: write `fn animal_chorus(animals: &[&dyn Animal])` that
//       calls .speak() on each element.
//       (uses a &dyn Animal — dynamic dispatch)

fn main() {
    let choir: [&dyn Animal; 3] = [/* … */];
    animal_chorus(&choir);
}
```

---

## Task 10.6 — Polymorphism with generics (static dispatch)

```rust
// TODO: write a generic function `fn volume<T: Volume>(t: &T) -> f64`
//       where trait Volume has: fn volume(&self) -> f64.

struct Cube(f64); struct Sphere(f64);
impl Volume for Cube {
    fn volume(&self) -> f64 { self.0 * self.0 * self.0 }
}
impl Volume for Sphere {
    fn volume(&self) -> f64 { 4.0 / 3.0 * std::f64::consts::PI * self.0.powi(3) }
}

fn main() {
    let shapes: [&dyn Volume; 2] = [&Cube(2.0), &Sphere(1.0)];
    for s in shapes { println!("volume = {:.2}", volume(s)); }
}
```

---

## Task 10.7 — Default trait methods + override

```rust
trait Player {
    fn name(&self) -> &str;
    fn play(&self) { println!("{} is just playing.", self.name()); }
}

struct Guitarist { name: &'static str, pedals: u8 }
struct Vocalist { name: &'static str, range: &'static str }

impl Player for Guitarist {
    fn name(&self) -> &str { self.name }
    // does NOT override play() → uses default from `Player`
}

impl Player for Vocalist {
    fn name(&self) -> &str { self.name }
    fn play(&self) { println!("{} hits {range} range!", self.name(), range = self.range) }
}

fn main() {
    let g = Guitarist { name: "Jimi", pedals: 5 };
    let v = Vocalist { name: "Freddie", range: "four-octave" };
    g.play();
    v.play();
    // expect:
    //   Jimi is just playing.
    //   Freddie hits four-octave range!
}
```

---

## Task 10.8 — When to use static vs dynamic dispatch

Fill in the blanks:

| When to use | Pick |
|---|---|
| Many different concrete types in a vec → runtime choice | `&dyn Trait` |
| Compile-time known type → maximum speed | `T: Trait` (generics) |
| Large number of short-lived types, no heterogenous vec needed | Generics |
| Plugin architecture / callback stored as value | `Box<dyn Trait>` |

Which situation would you choose **static dispatch** and why?

---

## Solutions

<details>
<summary>Click to reveal</summary>

```rust
// 10.1
trait Describable {
    fn describe(&self) -> &str;
}

struct Cat { name: &'static str, meow_pitch: u8 }

impl Describable for Cat {
    fn describe(&self) -> &str {
        format!("{} meows at {} Hz", self.name, self.meow_pitch)
    }
}

// 10.2 — add default to trait + override in Cat
trait Describable {
    fn describe(&self) -> &str;
    fn introduce(&self) { println!("Hi, I'm {}", self.describe()); }
}

impl Describable for Cat {
    fn describe(&self) -> &str { /* same as above */ }
    // introduce() → uses default
}

// 10.3
pub struct SavingsAccount {
    owner:    &'static str,
    balance:  u64,    // private
}

impl SavingsAccount {
    pub fn new(owner: &'static str, initial: u64) -> Self {
        Self { owner, balance: initial }
    }
    pub fn balance(&self) -> u64 { self.balance }
    pub fn deposit(&mut self, amt: u64) { self.balance += amt; }
}

// 10.4
pub struct Battery { capacity_mah: u32, current_mah: u32 }
impl Battery {
    pub fn new(cap: u32) -> Self { Self { capacity_mah: cap, current_mah: cap } }
    pub fn remaining(&self) -> u32 { self.current_mah }
    pub fn drain(&mut self, mah: u32) { self.current_mah = self.current_mah.saturating_sub(mah); }
    pub fn charge(&mut self) { self.current_mah = self.capacity_mah; }
}
pub struct Phone { model: &'static str, pub battery: Battery }
impl Phone {
    pub fn new(model: &'static str, cap: u32) -> Self { Self { model, battery: Battery::new(cap) } }
    pub fn talk(&mut self, mins: u32) { self.battery.drain(mins / 2); }
}

// 10.5
trait Animal { fn speak(&self); }
struct Dog { name: &'static str }
struct Bird { name: &'static str }
impl Animal for Dog  { fn speak(&self) { println!("{} says woof!", self.name); } }
impl Animal for Bird { fn speak(&self) { println!("{} says tweet!", self.name); } }

fn animal_chorus(animals: &[&dyn Animal]) {
    for a in animals { a.speak(); }
}

// 10.6
trait Volume { fn volume(&self) -> f64; }

// 10.7 — already in task, just verify output

// 10.8 — pick generics when concrete types are known at compile time
//         and you want monomorphised (inlined) calls.
```
</details>
