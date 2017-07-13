#![feature(custom_attribute, lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

// Defines various language items that need to be around
mod lang;
pub mod intrinsics;

extern crate compiler_builtins;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!++");
}
