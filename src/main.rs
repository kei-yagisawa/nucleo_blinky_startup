#![no_std]
#![no_main]

use nucleo_blinky_startup::reg;
use nucleo_blinky_startup::systick;

#[no_mangle]
pub fn main() -> ! {
    init_gpio();
    systick::init(|| {
        static mut ON_OFF: bool = false;
    
        let gpioa_odr = 0x4002_0014 as *mut usize;
        unsafe {
            ON_OFF = !ON_OFF;
            reg::set_bit(gpioa_odr, 5, ON_OFF);
        }
    });

    loop {}
}

fn init_gpio() {
    let rcc_ahb1enr = 0x4002_3830 as *mut usize;
    let gpioa_moder = 0x4002_0000 as *mut usize;

    reg::set_bit(rcc_ahb1enr,  0, true);
    reg::set_bit(gpioa_moder, 10, true);
}
