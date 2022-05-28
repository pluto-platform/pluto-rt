#![no_std]
#![no_main]

mod uart;
mod lib;
mod led;

extern crate panic_halt;

use core::arch::asm;
use riscv_rt::entry;
use nb;
use nb::block;
use uart::{Uart, UART};
use lib::{ecall, wait, enable_custom_interrupt};
use led::Led;

static MESSAGE: &str = "\r\nHello";

#[entry]
fn main() -> ! {

    let uart = Uart::init(0x20000, 50_000_000, 115_200);
    let led = Led::init(0x10000);

    ecall();

    println!("{} World!", MESSAGE);

    unsafe {
        riscv::register::mie::set_mext();
        enable_custom_interrupt(0);
        riscv::register::mstatus::set_mie();
    }

    loop {
        led.shift_left();
        print!("*");
        wait(5_000_000);
    }
}



fn clear_button_interrupt() {
    let button = 0x30000 as *mut u8;
    unsafe { button.write_volatile(0) }
}

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(_trap_frame: &riscv_rt::TrapFrame) {
    if riscv::register::mcause::read().code() == 11 {
        println!("ecall");
    } else {loop {}}
}

#[export_name = "MachineExternal"]
fn custom_external_handler() {
    clear_button_interrupt();
    println!("button");
}

#[export_name = "DefaultHandler"]
fn default_interrupt_handler() {
    if riscv::register::mcause::read().code() == 16 {
        UART.write_byte(UART.read_byte())
    }
}