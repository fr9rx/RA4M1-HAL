pub fn UnlockRegisters() {
    unsafe {
        let pmisc = &ra4m1_pac::PMISC;
        pmisc.pwpr().write_raw(0x40);
    }
} 