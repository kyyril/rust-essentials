# Exercise 13 — Macros

## Task 13.1 — Reimplement `println!` (simplified)

Write a macro `my_println!` that accepts:
```
my_println!("value = {}", x);
```
and produces `println!("[my_println] value = {x}");` using an internal helper.

```rust
macro_rules! my_println {
    // ── TODO ──
}

fn main() {
    let x = 42;
    my_println!("value = {}", x);
    // expect: [my_println] value = 42
}
```

---

## Task 13.2 — Variadic vec with default

Fix the macro below so it handles zero arguments correctly (compile two separate calls):

```rust
macro_rules! vec_macro {
    // ── TODO ── handle 0 args, 1+ args, trailing comma
}

fn main() {
    let a = vec_macro!();                  // expect []
    let b = vec_macro!(1);                 // expect [1]
    let c = vec_macro!(1, 2, 3);           // expect [1,2,3]
    let d = vec_macro!(4, 5, 6,);          // expect [4,5,6] — trailing comma ok
}
```

---

## Task 13.3 — DSL: SQL SELECT builder

Write a `sql!` macro that builds a `String`:

```rust
let query = sql!(SELECT * FROM users WHERE age >= 18 LIMIT 10);
// expects: "SELECT * FROM users WHERE age >= 18 LIMIT 10"
```

Hint: `stringify!($($tokens)*)` captures raw tokens as a string. Bonus: allow optional `ORDER BY`.

---

## Task 13.4 — `n_ary!` sum macro

```rust
macro_rules! n_ary {
    // ── TODO ──
}

fn main() {
    assert_eq!(n_ary!(1, 2, 3), 6);
    assert_eq!(n_ary!(10), 10);
}
```

---

## Task 13.5 — Attribute macro simulation

```rust
// TODO: define a `timed` macro that wraps a function body:
// prints "start <fn_name>" on entry and "done  <fn_name> (X ms)" on exit.
macro_rules! timed_fn {
    // ── TODO ──
}

timed_fn! { fn slow() { std::thread::sleep(std::time::Duration::from_millis(50)); } }
fn main() { slow(); }
```

---

## Answers

<details>
<summary>Click to reveal</summary>

```rust
// 13.1
macro_rules! my_println {
    ($($t:tt)*) => { println!("[my_println] {}", format!($($t)*)); };
}

// 13.2 — three rules needed
macro_rules! v {
    () => (vec![]);
    ($e:expr) => (vec![$e]);
    ($($e:expr),+ $(,)?) => (vec![$($e),+]);
}

// 13.3 — token-tree capture
macro_rules! sql {
    ($($t:tt)*) => { stringify!($($t)*).to_string() };
}

// 13.4
macro_rules! sum {
    () => (0);
    ($x:expr) => ($x);
    ($a:expr, $($rest:expr),+ $(,)?) => { $a + sum!($($rest),+) };
}

// 13.5 — wraps body with println! before and after
macro_rules! timed_fn {
    ($name:ident $( $args:tt )* $body:block) => {
        fn $name $( $args )* {
            println!("start {}", stringify!($name));
            $body
            println!("done  {}", stringify!($name));
        }
    };
}
```
</details>
