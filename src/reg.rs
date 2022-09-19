use core::ptr::{read_volatile, write_volatile};

pub fn set_bit(addr: *mut usize, bit: usize, is_set: bool) {
    unsafe {
        let val = read_volatile(addr);
        match is_set {
            true  => write_volatile(addr, val |  (1 << bit)),
            false => write_volatile(addr, val & !(1 << bit))
        }
    }
}
