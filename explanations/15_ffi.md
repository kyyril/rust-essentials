# 15 — FFI (Foreign Function Interface)

## Overview

FFI lets Rust call functions written in C (or any language exposing a C ABI) and vice-versa. **Safety boundary**: `unsafe` is required for every FFI call site.

---

## Calling C from Rust — `extern "C"`

```rust
extern "C" {
    fn strlen(s: *const u8) -> usize;
}

let text = b"hello\0"; // must be null-terminated
let len = unsafe { strlen(text.as_ptr()) };
```

### The `unsafe` fence

`unsafe { … }` promises to the compiler you've manually ensured:
- The pointer is valid, non-null, derefable.
- Null-terminated where required by C.
- Aliasing rules are preserved.

---

##导出Rust函数给C — `#[no_mangle]`

Remove name-mangling so C can find it by name in the compiled dylib/exe:

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Build:
```
cargo rustc -- -C prefer-dynamic          # dynamic lib
rustc --crate-type cdylib lib.rs          # static lib
```

### ABI strings

| ABI string | Use |
|---|---|
| `"C"` | Standard C calling convention (portable) |
| `"stdcall"` | Windows stdcall |
| `"fastcall"` | x86 fastcall |
| `"system"` | Platform default (C on Windows) |

---

## Strings across FFI — `CString` / `CStr`

| Direction | Type |
|---|---|
| Rust → C | `CString::new(s).unwrap().as_ptr()` |
| C → Rust | `CStr::from_ptr(ptr).to_str().unwrap()` |

`CString` is **null-terminated**; constructing it with an interior `\0` fails with `NulError`.
`CStr` is a borrowed view — no allocation.

---

## Opaque handles (the C ↔ Rust pointer)

C code can't read Rust structs — expose them as `void *` (opaque pointers):

```rust
#[repr(C)]
pub struct Handle { /* hidden fields */ }

#[no_mangle] pub extern "C" fn handle_new() -> *mut Handle {
    Box::into_raw(Box::new(Handle { /* … */ }))
}
#[no_mangle] pub extern "C" fn handle_free(p: *mut Handle) {
    if !p.is_null() { unsafe { drop(Box::from_raw(p)) } }
}
```

- `Box::into_raw` → gives C a pointer without freeing.
- `Box::from_raw` → re-takes ownership and drops it when called from Rust-side cleanup.

**Rule**: `Box::from_raw` must be called exactly **once** per `into_raw`.

---

## `#[repr(C)]` — layout compatibility

Rust may reorder or insert padding fields. Use `#[repr(C)]` to enforce **C layout** (field order = declaration order, padding after each field as C would):

```rust
#[repr(C)]
pub struct Point {
    x: f64, // 8 bytes
    y: f64, // 8 bytes
    // total 16 bytes — matches C `struct { double x; double y; }`
}
```

Without `#[repr(C)]` the Rust side could layout differently — use `memoffset` crate or `static_assertions` to verify.

---

## Dynamic loading — `libloading`

Load symbols at run time from a `.dll` / `.so`:

```rust
let lib = unsafe { Library::new("mylib.dll") }.expect("load");
unsafe {
    let func: Symbol<extern "C" fn(i32) -> i32> = lib.get(b"my_func\0");
    println!("{}", func(42));
}
```

---

## Safety summary

| Pattern | Safe? | Use |
|---|---|---|
| `extern "C"` call | ❌ `unsafe` | When calling C |
| `extern "C"` export | ✅ (if impl body safe) | C calls Rust |
| Opaque pointer | ✅ (boxed, not raw) | Allocate with Box |
| `CString` / `CStr` | ✅ (checked) | String cross FFI |
| `#[repr(C)]` struct | ✅ (no `unsafe{ }` needed) | Layout compatibility |
| `libloading` | ❌ `unsafe` | Load symbol & call |
