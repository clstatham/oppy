#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;

use crate::arch::{Arch, TargetArch};

pub mod arch;
#[macro_use]
pub mod serial;
pub mod logging;
pub mod memory;
pub mod util;

#[no_mangle]
pub extern "C" fn start() -> ! {
    logging::init();

    unsafe {
        TargetArch::init();
    }

    log::info!("It did not crash!");

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
