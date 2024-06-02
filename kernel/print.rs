use crate::dev;
use crate::dev::uart::{self, Uart, MALTA_SERIAL_BASE};
use core::fmt::{self, Write};
use core::panic;
pub struct Stdout {
    uart: uart::Ns16550a,
}

static mut STDOUT: Stdout = Stdout {
    uart: uart::Ns16550a::new(MALTA_SERIAL_BASE, 0),
};

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.uart.putchar(c as u32);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    unsafe { STDOUT.write_fmt(args).unwrap() };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*);
    };
}

#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panic at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panic at unknown location: {}", info.message().unwrap());
    }
    dev::halt();
    unreachable!("Then sentence should never be printed in panic.");
}
