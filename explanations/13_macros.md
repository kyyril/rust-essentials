# 13 — Macros

## Overview

Rust macros let you **write code that writes code** — programmable at compile time. There are three kinds:

| Kind | Invoked as | When it runs |
|---|---|---|
| Declarative | `my_vec!(…)` | Compile time |
| Function-like procedural | `my_macro!(…)` | Compile time (as a function) |
| Attribute | `#[my_attr]` | Compile time (attached to an item) |
| Derive | `#[derive(MyTrait)]` | Type-checking (auto trait impl) |

---

## Declarative macros — `macro_rules!`

The most common macro. A set of **pattern → replacement** rules.

```rust
macro_rules! say_hello {
    () => { println!("Hello!"); };
    ($name:expr) => { println!("Hello, {}!", $name); };
}
say_hello!();          // Hello!
say_hello!("Rust");    // Hello, Rust!
```

### Fragment specifiers

| Specifier | Matches |
|---|---|
| `expr` | Any expression |
| `ident` | An identifier (`foo`, `bar`) |
| `ty` | A type expression |
| `pat` | A pattern |
| `stmt` | A statement |
| `block` | A `{ … }` block |
| `tt` | A single token tree |
| `vis` | Visibility (`pub`, `pub(crate)`) |

Example: building a map literal DSL:
```rust
macro_rules! kv {
    ($($k:expr => $v:expr),+ $(,)?) => {
        { let mut m = HashMap::new(); $(m.insert($k, $v);)+ m }
    };
}
let ages = kv! { "Alice" => 30, "Bob" => 25 };
```

### Hygiene

Declarative macros are **lexically hygienic** — the identifiers they generate do not capture variables in the caller's scope. This prevents subtle bugs, unlike C preprocessor macros.

---

## Procedural macros

Declarative macros are powerful but limited to pattern-driven rewriting. **Procedural macros** run arbitrary Rust code at compile time — they operate on the **token stream** (`proc_macro::TokenStream`) directly.

Three flavours:

| Type | Example |
|---|---|
| Function-like | `let s = my_macro!(data);` |
| Attribute | `#[my_macro] fn foo() {}` |
| Derive | `#[derive(MyTrait)] struct Foo;` |

To write one, add `proc-macro = true` to your crate:
```
[lib]
proc-macro = true
```

A basic derive macro:
```rust
// in proc-macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyDerive)]
pub fn my_derive_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl #name {
            fn hello(&self) -> &str { stringify!(#name) }
        }
    };
    TokenStream::from(expanded)
}
```

### Proc-macro crates you will use in production

| Crate | Purpose |
|---|---|
| `syn` | Parse Rust source into a typed AST |
| `quote` | Format Rust code back into a `TokenStream` |
| `proc-macro2` | Wrapper that works in both proc-macro and non-proc contexts |
| `thiserror` / `anyhow` | Derive `Error` impl — not proc-macros but save massive boilerplate |

---

## Derive macros and `#[derive]`

`#[derive(Debug)]` is a **compiler-internal** procedural macro. The compiler knows trait bounds and generates `impl fmt::Debug`. Third-party derive macros (serde, derive-new, strum) follow the same pattern: they run **once per type definition** at compile time.

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct User {
    name: String,
    age: u32,
}
```

The order of `#[derive]` does not matter — each is independently applied.

---

## When to use macros?

| Situation | Macro or code? |
|---|---|
| `vec![1,2,3]` syntax | Macro |
| `println!` formatting | Macro (variable argument count) |
| `#[derive(Debug)]` | Derive macro / built-in |
| Repetitive boilerplate (getters, builders) | Derive macro |
| Compile-time computation | Procedural macro |
| Conditional compilation | `#[cfg(...)]` attribute |
| Performance-sensitive inlining | Macro generates concretely stamped fn |

---

## Execution order

```
Source file
   │
   ▼  macro expansion (token → token, repeat until fixed point)
   ▼  AST construction
   ▼  type checking
   ▼  borrow checking
   ▼  LLVM IR
   ▼  native code
```

Macros expand **before** type checking — so they cannot inspect or constrain types themselves. They work purely on tokens.

---

## Common pitfalls

| Problem | Fix |
|---|---|
| Macro repeats code and captures loop variable | Use `move` closure or unique generated variable names |
| Recursive expansion | Ensure base case is matched — otherwise "expansion limit exceeded" |
| Unwanted newlines/whitespace in output | Use `paste` crate or empty `!` lines |
| Debugging expansion | `cargo expand` (install `cargo-expand`) |
