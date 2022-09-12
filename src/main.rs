#![no_main]
#![no_std]
#![feature(llvm_asm)]

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
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
            llvm_asm!("" :::: "volatile")
        }
    }
}

unsafe fn set_bit( addr: *mut usize, bit: usize, is_set: bool ) {
    if is_set {
        *addr |= 1 << bit;
    }
    else {
        *addr &= !(1 << bit);
    }
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
