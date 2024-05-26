#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;

use crate::arch::{Arch, TargetArch};

pub mod arch;
#[macro_use]
pub mod serial;
pub mod logging;
pub mod util;

#[no_mangle]
pub extern "C" fn start() -> ! {
    TargetArch::init();

    logging::init();

    log::info!("This is a test info message");
    log::warn!("This is a test warning message");
    log::error!("This is a test error message");
    log::debug!("This is a test debug message");
    log::trace!("This is a test trace message");

    TargetArch::hcf();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;

    if let Some(mut serial) = serial::SERIAL0.try_lock() {
        let _ = serial.write_fmt(format_args!("{}\n", info));
    }

    TargetArch::hcf();
}
