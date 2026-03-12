pub struct OutputPin {
    pin: (u8, u8)
}

impl OutputPin {
    pub fn new(pin: (u8, u8)) -> Self {
        unsafe {
            match pin.0 {
                0 => {
                    let port0 = &ra4m1_pac::PORT0;
                    port0.pdr().write_raw(1 << pin.1);
                }

                1 => {
                    let port1 = &ra4m1_pac::PORT1;
                    port1.pdr().write_raw(1 << pin.1);
                }

                _ => {
                    loop {
                        
                    }
                }
            }
        }
        OutputPin { pin }
    }

    pub fn set_high(&self) {
        unsafe {
            match self.pin.0 {
                0 => {
                    let port0 = &ra4m1_pac::PORT0;
                    port0.posr().write_raw(1 << self.pin.1);
                }

                1 => {
                    let port1 = &ra4m1_pac::PORT1;
                    port1.posr().write_raw(1 << self.pin.1);
                }

                _ => {
                    loop {
                        
                    }
                }
            }
        }
        }

            pub fn set_low(&self) {
        unsafe {
            match self.pin.0 {
                0 => {
                    let port0 = &ra4m1_pac::PORT0;
                    port0.porr().write_raw(1 << self.pin.1);
                }

                1 => {
                    let port1 = &ra4m1_pac::PORT1;
                    port1.porr().write_raw(1 << self.pin.1);
                }

                _ => {
                    loop {
                        
                    }
                }
            }
        }
    }
    }
