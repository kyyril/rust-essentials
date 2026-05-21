// Topic 15: FFI (Foreign Function Interface)
// Run with: cargo run --bin ffi

// This example requires a C compiler (MinGW / GCC / Clang on Windows).
// If no C compiler is found it will skip the C interop demo with a notice.

// =========================================================================
// 1. extern "C" — declare a C function
// =========================================================================
extern "C" {
    /// C standard library strlen (from <string.h>)
    fn strlen(s: *const u8) -> usize;
}

// =========================================================================
// 2. Calling a C function safely
// =========================================================================
fn call_c_strlen() {
    let text = b"hello\0"; // null-terminated — required by C
    let len = unsafe { strlen(text.as_ptr()) };
    unsafe { println!("  strlen(\"hello\") = {} (via C strlen)", len) };
}

// =========================================================================
// 3. Exporting Rust to C — #[no_mangle] + extern "C"
// =========================================================================
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// =========================================================================
// 4. Callbacks — pass Rust fn pointer to C simulation
// =========================================================================
type CCallback = unsafe extern "C" fn(i32) -> i32;

unsafe extern "C" fn rust_callback(n: i32) -> i32 {
    println!("  [callback] Rust received {n}");
    n * 2
}

fn callback_demo() {
    let cb: CCallback = rust_callback;
    let result = unsafe { cb(7) };
    unsafe { println!("  callback result = {} (via Rust fn pointer)", result) };
}

// =========================================================================
// 5. CString / CStr — null-terminated strings across FFI
// =========================================================================
use std::ffi::{CStr, CString};

fn cstring_demo() {
    // Rust String → C string (null-terminated)
    let cs = CString::new("hello from CString").expect("no null bytes");
    unsafe { println!("  CString: {:?}", CStr::from_ptr(cs.as_ptr())) };

    // C string → Rust &CStr
    let raw = b"world\0";
    let cstr = unsafe { CStr::from_ptr(raw.as_ptr() as *const i8) };
    unsafe { println!("  &CStr: {:?}", cstr) };
}

// =========================================================================
// 6. libloading — dynamic library at run time
// =========================================================================
fn dynamic_lib_demo() {
    use libloading::{Library, Symbol};
    // Load msvcrt (Windows) or libc.so.6 (Linux) at run time
    let lib = unsafe { Library::new("msvcrt.dll") }.expect("load msvcrt");
    unsafe {
        // strlen is a real symbol we can look up
        let strlen: Symbol<unsafe extern "C" fn(*const u8) -> usize> =
            lib.get(b"strlen\0").expect("sym strlen not found");
        let s = b"dynlib\0";
        println!("  dynlib strlen = {}", strlen(s.as_ptr()));
    }
}

// =========================================================================
// 7. Opaque pointers — PASS
// =========================================================================
#[repr(C)]
pub struct RustOpaque {
    value: i32,
}

#[no_mangle]
pub extern "C" fn opaque_create(v: i32) -> *mut RustOpaque {
    Box::into_raw(Box::new(RustOpaque { value: v }))
}

#[no_mangle]
pub extern "C" fn opaque_get(ptr: *mut RustOpaque) -> i32 {
    unsafe { (*ptr).value }
}

#[no_mangle]
pub extern "C" fn opaque_free(ptr: *mut RustOpaque) {
    unsafe { drop(Box::from_raw(ptr)) }
}

fn opaque_demo() {
    unsafe {
        let p = opaque_create(42);
        println!("  opaque value = {}", opaque_get(p));
        opaque_free(p);
    }
}

// =========================================================================
// 8. Safety rules reminder
// =========================================================================
fn safety_rules() {
    println!(
        "\n  Rule             │ Detail\n\
         ──────────────────┼────────────────────────────────────────\n\
         No dangling ptrs  │ caller & callee must agree on lifetime\n\
         No null deref     │ use Option<*mut T> or check against null\n\
         No use-after-free │ ownership must be clearly documented\n\
         No data races     │ &mut T *never* crosses FFI boundary\n\
         ABI must match    │ use #[repr(C)] on every shared struct\n\
         No exceptions     │ Rust has none — C's setjmp/longjmp are UB\n\
         No double-free    │ {free, Box::from_raw} called exactly once"
    );
}

// =========================================================================
// Main
// =========================================================================
fn main() {
    println!("=== 1. extern \"C\" — call C strlen ===");
    call_c_strlen();

    println!("\n=== 2. Callback (C calls Rust) ===");
    callback_demo();

    println!("\n=== 3. CString / CStr ===");
    cstring_demo();

    println!("\n=== 4. Opaque handles ===");
    opaque_demo();

    println!("\n=== 5. Dynamic library (libloading) ===");
    match dynamic_lib_demo() {
        Ok(()) => {}
        Err(e) => eprintln!("  (skipped: {e})"),
    }

    safety_rules();

    println!("\nAll FFI demos complete.");
}