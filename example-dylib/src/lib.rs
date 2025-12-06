//!An example dynamically loadable library.
//!
//! This crate creates a dynamic library that can be used for testing purposes.
//! It exports multiple symbols with different types and abis.
//! It's main purpose is to be used in tests of dynlib crate.

use std::os::raw::{c_char, c_int};

//FUNCTIONS
#[unsafe(no_mangle)]
pub fn rust_fun_print_something() {
    println!("something");
}

#[unsafe(no_mangle)]
pub fn rust_fun_add_one(arg: i32) -> i32 {
    arg + 1
}

#[unsafe(no_mangle)]
pub extern "C" fn c_fun_print_something_else() {
    println!("something else");
}

#[unsafe(no_mangle)]
pub extern "C" fn c_fun_add_two(arg: c_int) -> c_int {
    arg + 2
}

#[allow(unused_variables)]
#[unsafe(no_mangle)]
pub extern "C" fn c_fun_variadic(txt: *const c_char) {
    //pretend to be variadic - impossible to do in Rust code
}

//STATIC DATA
#[unsafe(no_mangle)]
pub static mut rust_i32_mut: i32 = 42;
#[unsafe(no_mangle)]
pub static rust_i32: i32 = 43;

#[unsafe(no_mangle)]
pub static mut c_int_mut: c_int = 44;
#[unsafe(no_mangle)]
pub static c_int: c_int = 45;

#[repr(C)]
pub struct SomeData {
    first: c_int,
    second: c_int,
}

#[unsafe(no_mangle)]
pub static c_struct: SomeData = SomeData {
    first: 1,
    second: 2,
};

//STATIC STRINGS

//exporting str directly is not so easy - it is not Sized!
//you can only export a reference to str and this requires double dereference
#[unsafe(no_mangle)]
pub static rust_str: &str = "Hello!";

#[unsafe(no_mangle)]
pub static c_const_char_ptr: [u8; 4] = [b'H', b'i', b'!', 0];
