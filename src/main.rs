#![no_main]
#![no_std]

use core::ptr::{copy_nonoverlapping, read_volatile, write_bytes, write_volatile};

extern crate panic_halt;

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
    Some(SysTick),
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
    init_systick();

    loop {}
}

#[no_mangle]
extern "C" fn SysTick() {
    static mut ON_OFF: bool = false;

    let gpioa_odr = 0x4002_0014 as *mut usize;
    unsafe {
        ON_OFF = !ON_OFF;
        set_bit(gpioa_odr, 5, ON_OFF);
    }
}

fn init_gpio() {
    let rcc_ahb1enr = 0x4002_3830 as *mut usize;
    let gpioa_moder = 0x4002_0000 as *mut usize;

    set_bit(rcc_ahb1enr, 0, true);
    set_bit(gpioa_moder, 10, true);
}

fn init_systick() {
    let stk_ctrl_addr = 0xE000_E010 as *mut usize;
    let stk_load_addr = 0xE000_E014 as *mut usize;
    let stk_val_addr  = 0xE000_E018 as *mut usize;

    unsafe {
        write_volatile(stk_val_addr, 0);
        write_volatile(stk_load_addr, 1_000_000);
        write_volatile(stk_ctrl_addr, 0x3);
    }
}

fn set_bit(addr: *mut usize, bit: usize, is_set: bool) {
    unsafe {
        let val = read_volatile(addr);
        match is_set {
            true  => write_volatile(addr, val |  (1 << bit)),
            false => write_volatile(addr, val & !(1 << bit))
        }
    }
}
