use lazy_static::lazy_static;
use uart_16550::SerialPort;

use crate::{arch::without_interrupts, util::lock::IrqMutex};

pub const SERIAL0_IOPORT: u16 = 0x3f8;
pub const SERIAL1_IOPORT: u16 = 0x2f8;
pub const SERIAL2_IOPORT: u16 = 0x3e8;

lazy_static! {
    pub static ref SERIAL0: IrqMutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL0_IOPORT) };
        serial_port.init();

        IrqMutex::new(serial_port)
    };
    pub static ref SERIAL1: IrqMutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL1_IOPORT) };
        serial_port.init();

        IrqMutex::new(serial_port)
    };
    pub static ref SERIAL2: IrqMutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL2_IOPORT) };
        serial_port.init();

        IrqMutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print0(args: core::fmt::Arguments) {
    use core::fmt::Write;

    without_interrupts(|| {
        SERIAL0.lock().write_fmt(args).unwrap();
    })
}

#[doc(hidden)]
pub fn _print1(args: core::fmt::Arguments) {
    use core::fmt::Write;

    without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).unwrap();
    })
}

#[doc(hidden)]
pub fn _print2(args: core::fmt::Arguments) {
    use core::fmt::Write;

    without_interrupts(|| {
        SERIAL2.lock().write_fmt(args).unwrap();
    })
}

#[macro_export]
macro_rules! serial0_print {
    ($($arg:tt)*) => {
        $crate::serial::_print0(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial1_print {
    ($($arg:tt)*) => {
        $crate::serial::_print1(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial2_print {
    ($($arg:tt)*) => {
        $crate::serial::_print2(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial0_println {
    () => {
        $crate::serial0_print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::serial0_print!("{}\n", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial1_println {
    () => {
        $crate::serial1_print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::serial1_print!("{}\n", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial2_println {
    () => {
        $crate::serial2_print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::serial2_print!("{}\n", format_args!($($arg)*));
    };
}
