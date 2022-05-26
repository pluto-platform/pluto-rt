#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::asm;
use riscv_rt::entry;

//static MESSAGE: &str = "\r\nHello!";

#[entry]
fn main() -> ! {
    let led = 0x10000 as *mut u8;

    let mut state = false;

    set_period(50_000_000/115_200);

    unsafe { asm!("ecall") }

    unsafe {
        riscv::register::mie::set_mext();
        enable_custom_interrupt(0);
        riscv::register::mstatus::set_mie();
    }


    loop {
        unsafe { led.write_volatile(state as u8) }
        if state {
            send_letter('*');
        } else {
            send_letter('_');
        }
        state = !state;
        wait(10_000_000); //2000000

    }
}

unsafe fn enable_custom_interrupt(i: u8) {
    asm!("csrrs x0, mie, {0}", in(reg) 1usize << (i+16))
}

fn wait(cycles: u32) {
    (0..cycles).for_each(|_| unsafe { asm!("nop") } );
}

fn send_letter(letter: char) {
    let uart = 0x20008 as *mut u8;
    while !is_ready() { unsafe { asm!("nop") } }
    unsafe{ uart.write_volatile(letter as u8) }
}
fn is_ready() -> bool {
    let uart = 0x20000 as *mut u8;
    unsafe{ uart.read_volatile() & 0x02 == 0x02 }
}
fn set_period(period: u32) {
    let uart = 0x20004 as *mut u32;
    unsafe { uart.write_volatile(period) }
}
fn get_letter() -> char {
    let uart = 0x20008 as *mut char;
    unsafe { uart.read_volatile() }
}

fn clear_button_interrupt() {
    let button = 0x30000 as *mut u8;
    unsafe { button.write_volatile(0) }
}

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) {
    if riscv::register::mcause::read().code() == 11 {
        send_letter('e');
        send_letter('c');
        send_letter('a');
        send_letter('l');
        send_letter('l');
    } else {loop {}}
}

#[export_name = "MachineExternal"]
fn custom_external_handler() {
    clear_button_interrupt();
    send_letter('b');
    send_letter('u');
    send_letter('t');
    send_letter('t');
    send_letter('o');
    send_letter('n');
}

#[export_name = "DefaultHandler"]
fn default_interrupt_handler() {
    if riscv::register::mcause::read().code() == 16 {
        send_letter(get_letter());
    }
}