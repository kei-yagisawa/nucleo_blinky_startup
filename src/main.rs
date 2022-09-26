#![no_std]
#![no_main]

use nucleo_blinky_startup::led;
use nucleo_blinky_startup::systick;

#[no_mangle]
pub fn main() -> ! {
    led::init();

    let mut st = systick::take();
    st.set_current(0);
    st.set_reload(1_000_000);
    st.set_interrupt(Some(|| {
        static mut ON_OFF: bool = false;

        unsafe {
            ON_OFF = !ON_OFF;
            led::set(ON_OFF);
        }
    }));

    let _ = st.start();
//    let st = st.start();
//    st.stop();

    loop {}
}
