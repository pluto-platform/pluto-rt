#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::asm;
use riscv_rt::entry;

static MESSAGE: &str = "\r\nHello!";

#[entry]
fn main() -> ! {
    let raw = 0x10000 as *mut u8;

    let mut state = true;
    loop {
        for letter in MESSAGE.chars() {
            send_letter(letter);
        }
        unsafe { raw.write_volatile(state as u8) }
        state = !state;
        wait(2000000); //2000000
    }
}


fn wait(cycles: u32) {
    for _ in 0..cycles {
        unsafe { asm!("nop") }
    }
}

fn send_letter(letter: char) {
    let uart = 0x20000 as *mut u8;
    while !is_ready() { unsafe { asm!("nop") } }
    unsafe{ uart.write_volatile(letter as u8) }
}
fn is_ready() -> bool {
    let uart = 0x20000 as *mut u8;
    unsafe{ uart.read_volatile() == 1 }
}
