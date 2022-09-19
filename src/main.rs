#![no_main]
#![no_std]

extern crate panic_halt;

mod reg;
mod systick;

use core::ptr::{copy_nonoverlapping, write_bytes};

#[link_section = ".vector_table.vectors"]
#[no_mangle]
static VECTORS: [Option<unsafe extern "C" fn()>; 15] = [
    Some(Reset),
    Some(DefaultExceptionHandler),
    Some(DefaultExceptionHandler),
    Some(DefaultExceptionHandler),
    Some(DefaultExceptionHandler),
    Some(DefaultExceptionHandler),
    None,
    None,
    None,
    None,
    Some(DefaultExceptionHandler),
    None,
    None,
    Some(DefaultExceptionHandler),
    Some(systick::SysTick),
];

#[no_mangle]
extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn Reset() {
    extern "C" {
        static mut _sbss  : u8;
        static     _ebss  : u8;
        static     _sidata: u8;
        static mut _sdata : u8;
        static     _edata : u8;
    }

    // 初期値なし変数の初期化
    let count = (&_ebss as *const u8).offset_from(&_sbss as *const u8) as usize;
    write_bytes(&mut _sbss, 0, count);

    // 初期値付き変数の初期化
    let count = (&_edata as *const u8).offset_from(&_sdata as *const u8) as usize;
    copy_nonoverlapping(&_sidata as *const u8, &mut _sdata, count);

    init_gpio();
    systick::init();

    loop {}
}

fn init_gpio() {
    let rcc_ahb1enr = 0x4002_3830 as *mut usize;
    let gpioa_moder = 0x4002_0000 as *mut usize;

    reg::set_bit(rcc_ahb1enr,  0, true);
    reg::set_bit(gpioa_moder, 10, true);
}
