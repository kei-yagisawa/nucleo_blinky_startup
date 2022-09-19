#![no_std]
#![no_main]

use nucleo_blinky_startup::led;
use nucleo_blinky_startup::systick;

#[no_mangle]
pub fn main() -> ! {
    led::init();

    systick::init(|| {
        static mut ON_OFF: bool = false;
    
        unsafe {
            ON_OFF = !ON_OFF;
            led::set(ON_OFF);
        }
    });

    loop {}
}
