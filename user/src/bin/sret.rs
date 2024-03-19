#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    println!("Try to execute a privileged instruction in User Mode.");
    println!("Kernel should kill this application!");
    unsafe {
        asm!("sret");
    }
    0
}
