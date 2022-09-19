use crate::reg;

pub fn init() {
    let rcc_ahb1enr = 0x4002_3830 as *mut usize;
    let gpioa_moder = 0x4002_0000 as *mut usize;

    reg::set_bit(rcc_ahb1enr,  0, true);
    reg::set_bit(gpioa_moder, 10, true);
}

pub fn set( is_set: bool ) {
    let gpioa_odr = 0x4002_0014 as *mut usize;

    reg::set_bit(gpioa_odr, 5, is_set);
}