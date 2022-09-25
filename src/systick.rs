use volatile_register::{RO, RW};

#[repr(C)]
struct SysTickReg {
    pub csr:   RW<u32>,
    pub rvr:   RW<u32>,
    pub cvr:   RW<u32>,
    pub calib: RO<u32>,
}

pub struct SysTick {
    reg: &'static mut SysTickReg,
}

impl SysTick {
    pub fn new() -> SysTick {
        SysTick {
            reg: unsafe { &mut *(0xE000_E010 as *mut SysTickReg) },
        }
    }

    pub fn start(&mut self) {
        unsafe {
            let csr = self.reg.csr.read();
            self.reg.csr.write(csr | 0x1);
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            let csr = self.reg.csr.read();
            self.reg.csr.write(csr & !0x1);
        }
    }

    pub fn set_interrupt(&mut self, cb: Option<fn()>) {
        unsafe {
            CB = cb;

            let csr = self.reg.csr.read();
            match cb {
                Some(_) => self.reg.csr.write(csr |  0x2),
                None    => self.reg.csr.write(csr & !0x2),
            }
        }
    }

    pub fn get_current(&self) -> u32 {
        self.reg.cvr.read()
    }

    pub fn set_current(&mut self, current_value: u32) {
        unsafe {
            self.reg.cvr.write(current_value);
        }
    }

    pub fn set_reload(&mut self, reload_value: u32) {
        unsafe {
            self.reg.rvr.write(reload_value);
        }
    }
}

static mut CB: Option<fn()> = None;

#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    match CB {
        Some(f) => f(),
        None    => (),
    }
}
