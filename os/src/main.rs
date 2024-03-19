#![no_main]
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;
use log::*;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn start_text();
        fn end_text();
        fn start_rodata();
        fn end_rodata();
        fn start_data();
        fn end_data();
        fn start_bss();
        fn end_bss();
        fn boot_stack_lower_bound();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();

    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        start_text as usize,
        end_text as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        start_rodata as usize, end_rodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        start_data as usize, end_data as usize
    );
    warn!(
        "[kernel] boot_stack top={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x})",
        start_bss as usize, end_bss as usize
    );

    sbi::shutdown(false);
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize)
        .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
}
