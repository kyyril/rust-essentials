# Exercise 15 — FFI

## Task 15.1 — safe wrapper around `strlen`

```rust
extern "C" { fn strlen(s: *const u8) -> usize; }

fn main() {
    let text = b"hello\0";
    unsafe { println!("{}", strlen(text.as_ptr())) };
}

fn c_strlen(s: &str) -> usize {
    // TODO: convert &str to *const u8 and call strlen
}
```

---

## Task 15.2 — Box::into_raw round-trip

```rust
#[repr(C)] struct Foo { value: i32 }

// TODO: fn foo_new(v: i32) -> *mut Foo   — allocate with Box::into_raw
// TODO: fn foo_get(p: *mut Foo) -> i32    — safe read
// TODO: fn foo_free(p: *mut Foo)          — deallocate with Box::from_raw (exactly once)

fn main() {
    let p = foo_new(42);
    assert_eq!(foo_get(p), 42);
    unsafe { foo_free(p) };
}
```

---

## Task 15.3 — call a C function at run-time

```rust
use libloading::{Library, Symbol};

fn main() {
    // Load msvcrt.dll and look up strlen at run-time
    // TODO: 1) Library::new("msvcrt.dll")
    //       2) lib.get(b"strlen\0")
    //       3) call through the Symbol and print result
}
```

---

## Task 15.4 — #[repr(C)] layout check

```rust
#[repr(C)] struct TwoD { x: f64, y: f64 }

fn main() {
    // TODO: verify &TwoD { x: 1.0, y: 2.0 } is 16 bytes exactly.
    // Hint: std::mem::size_of_val(&s). No unsafe needed.
}
```

---

## Answers

<details>
<summary>Click to reveal</summary>

```rust
// 15.1
fn c_strlen(s: &str) -> usize {
    unsafe { strlen(s.as_ptr()) }
}

// 15.2
fn foo_new(v: i32) -> *mut Foo {
    Box::into_raw(Box::new(Foo { value: v }))
}
unsafe fn foo_get(p: *mut Foo) -> i32 { (*p).value }
unsafe fn foo_free(p: *mut Foo) { drop(Box::from_raw(p)) }

// 15.3
let lib = unsafe { Library::new("msvcrt.dll") }.unwrap();
let strlen: Symbol<unsafe extern "C" fn(*const u8) -> usize> = unsafe { lib.get(b"strlen\0") }.unwrap();
let s = b"dyn-call\0";
println!("{}", strlen(s.as_ptr()));

// 15.4
let s = TwoD { x: 1.0, y: 2.0 };
assert_eq!(std::mem::size_of_val(&s), 16);
```
</details>
