use crate::dev;
use crate::dev::uart::Uart;
use crate::dev::uart::NS16550A;
use core::fmt::{self, Write};
use core::panic;
use lazy_static::lazy_static;
use sync::spin::Spinlock;
pub struct Stdout;

lazy_static! {
    static ref STDOUT: Spinlock<Stdout> = Spinlock::new(Stdout {});
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            NS16550A.putchar(c as u32);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
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
}
