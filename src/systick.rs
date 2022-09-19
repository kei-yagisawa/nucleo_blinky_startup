use core::ptr::write_volatile;

static mut CB: Option<fn()> = None;

#[no_mangle]
pub extern "C" fn SysTick() {
    match unsafe { CB } {
        Some(f) => f(),
        None    => ()
    }
}

pub fn init( f: fn() ) {
    unsafe {
        CB = Some(f);
    
        let stk_ctrl_addr = 0xE000_E010 as *mut usize;
        let stk_load_addr = 0xE000_E014 as *mut usize;
        let stk_val_addr  = 0xE000_E018 as *mut usize;

        write_volatile(stk_val_addr, 0);
        write_volatile(stk_load_addr, 1_000_000);
        write_volatile(stk_ctrl_addr, 0x3);
    }
}
