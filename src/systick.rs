use volatile_register::{RO, RW};

#[repr(C)]
struct SysTickReg {
    pub csr:   RW<u32>,
    pub rvr:   RW<u32>,
    pub cvr:   RW<u32>,
    pub calib: RO<u32>,
}

pub struct Enabled;
pub struct Disabled;

#[allow(dead_code)]
pub struct SysTick<EN> {
    reg    : &'static mut SysTickReg,
    enabled: EN,
}

impl SysTick<Disabled> {
    pub fn start(self) -> SysTick<Enabled> {
        unsafe {
            let csr = self.reg.csr.read();
            self.reg.csr.write(csr | 0x1);
        }

        SysTick {
            reg    : self.reg,
            enabled: Enabled
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

impl SysTick<Enabled> {
    pub fn stop(self) -> SysTick<Disabled> {
        unsafe {
            let csr = self.reg.csr.read();
            self.reg.csr.write(csr & !0x1);
        }

        SysTick {
            reg    : self.reg,
            enabled: Disabled
        }
    }
}

pub fn take() -> SysTick<Disabled> {
    SysTick {
        reg    : unsafe { &mut *(0xE000_E010 as *mut SysTickReg) },
        enabled: Disabled
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
