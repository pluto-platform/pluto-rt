#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::asm;
use nb;
use riscv_rt::entry;
use embedded_hal::serial::*;
use nb::block;

static MESSAGE: &str = "\r\nHello!";

#[entry]
fn main() -> ! {
    let mut serial = UART { address: 0x20000 };
    let raw = 0x10000 as *mut u8;

    let mut state = false;

    let _val = block!(serial.read()).unwrap();

    println!("{} World!",MESSAGE);

    loop {
        for letter in MESSAGE.chars() {
            block!(serial.write(letter as u8)).unwrap();
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

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(UART { address: 0x20000 }, $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}


struct UART {
    address: usize
}

impl UART {
    fn has_data(&self) -> bool {
        unsafe {
            (self.address as *mut u8).read_volatile() & 0x01 == 0x01
        }
    }
    fn ready(&self) -> bool {
        unsafe {
            (self.address as *mut u8).read_volatile() & 0x02 == 0x02
        }
    }
    fn set_divider(&self, period: u32) {
        unsafe {
            ((self.address + 4) as *mut u32).write_volatile(period)
        }
    }
    fn read_byte(&self) -> u8 {
        unsafe {
            ((self.address + 8) as *mut u8).read_volatile()
        }
    }
    fn write_byte(&self, value: u8) {
        unsafe {
            ((self.address + 8) as *mut u8).write_volatile(value)
        }
    }
}

enum SerialError {
    Overrun,
    Parity
}

impl Read<u8> for UART {
    type Error = ();
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.has_data() {
            Ok(self.read_byte())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for UART {
    type Error = ();
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.ready() {
            self.write_byte(word);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}

impl core::fmt::Write for UART {
    // The trait Write expects us to write the function write_str
    // which looks like:
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for c in s.bytes() {
            block!(self.write(c)).unwrap();
        }
        // Return that we succeeded.
        Ok(())
    }
}