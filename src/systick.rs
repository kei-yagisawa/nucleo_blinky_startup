use core::ptr::write_volatile;
use crate::reg;

#[no_mangle]
pub extern "C" fn SysTick() {
    static mut ON_OFF: bool = false;

    let gpioa_odr = 0x4002_0014 as *mut usize;
    unsafe {
        ON_OFF = !ON_OFF;
        reg::set_bit(gpioa_odr, 5, ON_OFF);
    }
}

pub fn init() {
    let stk_ctrl_addr = 0xE000_E010 as *mut usize;
    let stk_load_addr = 0xE000_E014 as *mut usize;
    let stk_val_addr  = 0xE000_E018 as *mut usize;

    unsafe {
        write_volatile(stk_val_addr, 0);
        write_volatile(stk_load_addr, 1_000_000);
        write_volatile(stk_ctrl_addr, 0x3);
    }
}
