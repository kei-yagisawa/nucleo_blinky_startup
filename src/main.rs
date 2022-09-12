#![no_main]
#![no_std]
#![feature(asm)]

extern crate panic_halt;


#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub extern "C" fn Reset() -> ! {
    let rcc_ahb1enr = 0x4002_3830 as *mut usize;
    let gpioa_moder = 0x4002_0000 as *mut usize;
    let gpioa_odr   = 0x4002_0014 as *mut usize;

    set_bit(rcc_ahb1enr,  0, true);
    set_bit(gpioa_moder, 10, true);
    
    let mut flg: bool = true;
    loop {
        flg = !flg;
        set_bit(gpioa_odr, 5, flg);

        for _ in 0..24_000 {
            unsafe { asm!("nop"); }
        }
    }
}


fn set_bit( addr: *mut usize, bit: usize, is_set: bool ) {
    unsafe {
        match is_set {
            true  => *addr |= 1 << bit,
            false => *addr &= !(1 << bit)
        }
    }
}
