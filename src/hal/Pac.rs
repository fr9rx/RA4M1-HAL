pub struct Pins {
    pub D13: (u8, u8)
}

pub struct Prephipals {
    pub pins: Pins,
}

static mut taken: bool = false;

impl Prephipals {
    pub fn take() -> Self {
        unsafe {
        if taken {
            loop {
                
            }
        }
        else {
            taken = true;
            Self {
                pins: Pins {
                    D13: (1, 2)
                },
            }
        }
    }
}
}