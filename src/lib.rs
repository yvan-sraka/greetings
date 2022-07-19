//! This is a dumb library that just expose a `hello` function that take an
//! input and will print it to the standard output with the canonical greeting
//! formatting, e.g. `hello("World")` will write `Hello, World!` on
//! `/dev/stdout`.
//!
//! The Book have sections about FFI in the unsafe chapter "Calling Rust
//! Functions from Other Languages"
//! https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-rust-functions-from-other-languages
//!
//! This project assumes that:
//!
//! -   You're running a modern Linux machine (it does not have been tested on
//!     other environment).
//!
//! -   You have both [Nix](https://nixos.org/download.html) package manager
//!     installed on your machine and available in your `$PATH` and
//!     [`direnv`](https://direnv.net/docs/hook.html) hooked in your shell.
//!
//! ```shell
//! cargo build # build the library (`libgreetings.a`)
//! ```
//!
//! For more high-level (type-preserving) bindings, give a try to
//! [curryrs](https://github.com/mgattozzi/curryrs) from
//! [Michael Gattozzi](https://twitter.com/mgattozzi) that wrote a couple of
//! articles on [FFI with Haskell and Rust](https://blog.mgattozzi.dev/haskell-rust/)
//! that I read along the way of this exercise.

use std::{ffi::CStr, os::raw::c_char};

/// This function does not require an extensive documentation to understand
/// what it does, but it's always nice to point out that everything is safe
/// here :)
pub fn hello(input: &str) {
    println!("Hello, {}!", input);
}

/// A wrapper to `hello` function using `extern "C"` ABI call conventions.
///
/// There is several design consideration to have in mind:
///
/// -   `String` in Rust are UTF-8 which in not the case in C where `char*`
///     encode well ASCII
///
/// -   We have to use an FFI-safe type as function argument. Which would say
///     that the compiler guarantee that a struct have specified layout (memory
///     representation) by having an `#[repr(C)]` attribute, which is the case
///     of `c_char` but not of `CStr`.
///
/// # Safety
///
/// This function need the data behind `input` pointer to be a valid
/// C-compatible, nul-terminated string with no nul bytes in the middle as
/// described in https://doc.rust-lang.org/std/ffi/struct.CString.html#safety
/// Otherwise, this function will panic.
#[no_mangle] // Mangling randomize symbols, and we need to preserve `c_hello`
pub unsafe extern "C" fn c_hello(input: *const c_char) {
    hello(CStr::from_ptr(input).to_str().expect("invalid UTF-8 data"));
    // TODO: display `std::str::Utf8Error` information in panic message
}
