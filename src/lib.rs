#![no_std]
use core::arch::asm;
use core::iter::Iterator;


pub fn ecall() {
    unsafe { asm!("ecall") }
}

pub fn wait(cycles: u32) {
    (0..cycles).for_each(|_| unsafe { asm!("nop") } );
}

pub unsafe fn enable_custom_interrupt(i: u8) {
    asm!("csrrs x0, mie, {0}", in(reg) 1usize << (i+16))
}