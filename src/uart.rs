
use embedded_hal::serial::*;
use nb;
use nb::block;


pub struct Uart {
    pub address: usize
}

pub static UART: Uart = Uart { address: 0x20000 };

impl Uart {

    pub fn init(address: usize, board_frequency: u32, baud_rate: u32) -> Uart {
        let uart = Uart { address };
        uart.set_divider(board_frequency / baud_rate);
        uart
    }

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
    pub fn read_byte(&self) -> u8 {
        unsafe {
            ((self.address + 8) as *mut u8).read_volatile()
        }
    }
    pub fn write_byte(&self, value: u8) {
        unsafe {
            ((self.address + 8) as *mut u8).write_volatile(value)
        }
    }

}

enum SerialError {
    Overrun,
    Parity
}

impl Read<u8> for Uart {
    type Error = ();
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.has_data() {
            Ok(self.read_byte())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for Uart {
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

impl core::fmt::Write for Uart {
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

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(Uart { address: 0x20000 }, $($args)+);
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
