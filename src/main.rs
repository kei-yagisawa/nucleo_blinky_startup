#![no_std]
#![no_main]

use nucleo_blinky_startup::led;
use nucleo_blinky_startup::systick;

#[no_mangle]
pub fn main() -> ! {
    led::init();

    let mut st = systick::SysTick::new();
    st.set_current(0);
    st.set_reload(1_000_000);
    st.set_interrupt(Some(|| {
        static mut ON_OFF: bool = false;

        unsafe {
            ON_OFF = !ON_OFF;
            led::set(ON_OFF);
        }
    }));
    st.start();

    loop {}
}
