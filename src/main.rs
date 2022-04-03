#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::asm;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let raw = 0x10000 as *mut u8;
    unsafe{ core::ptr::write_volatile(raw, 0xFF) }
    unsafe { asm!("ecall"); }
    loop{}
}
