#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::asm;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let raw = 0x10000 as *mut u8;
    let mut state = 0u8;
    loop{
        unsafe{ raw.write_volatile(state) }
        state = !state;
        wait(100);
    }
}

fn wait(cycles: u32) {
    for _ in 0..cycles {
        unsafe { asm!("nop") }
    }
}