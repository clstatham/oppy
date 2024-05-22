#![no_std]
#![no_main]

use core::panic::PanicInfo;

use x86_64::instructions;

#[macro_use]
pub mod serial;
pub mod logging;
pub mod util;

#[no_mangle]
pub extern "C" fn start() -> ! {
    logging::init();

    log::info!("This is a test info message");
    log::warn!("This is a test warning message");
    log::error!("This is a test error message");
    log::debug!("This is a test debug message");
    log::trace!("This is a test trace message");

    hcf();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;

    if let Some(mut serial) = serial::SERIAL0.try_lock() {
        let _ = serial.write_fmt(format_args!("{}\n", info));
    }

    hcf();
}

pub fn hcf() -> ! {
    loop {
        instructions::hlt();
        core::hint::spin_loop();
    }
}
