#![no_std]
#![no_main]

use core::panic::PanicInfo;

use x86_64::instructions::hlt;

#[macro_use]
pub mod serial;
pub mod util;

#[no_mangle]
pub extern "C" fn start() -> ! {
    serial0_println!("Hello, world!");
    hcf();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn hcf() -> ! {
    loop {
        hlt();
        core::hint::spin_loop();
    }
}
