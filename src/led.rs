

pub struct Led {
    address: usize
}

impl Led {

    pub fn init(address: usize) -> Led {
        let led = Led { address };
        led.set(1);
        led
    }

    fn set(&self, state: u8) {
        unsafe { (self.address as *mut u8).write_volatile(state); }
    }
    fn get(&self) -> u8 {
        unsafe { (self.address as *mut u8).read_volatile() }
    }

    pub fn shift_left(&self) {
        self.set(self.get().rotate_left(1));
    }

}

