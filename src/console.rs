use core::panic::PanicInfo;
use core::fmt::{self,Write};
use crate::sbi;
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}

pub struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, s :&str) -> fmt::Result {
        for c in s.chars() {
            sbi::putchar(c as usize);
        }
        Ok(())
    }

}


#[macro_export]
macro_rules! print {
    ($fmt: literal $(,$($arg : tt) +)?) => {
        use core::fmt::Write;
        $crate::console::Stdout.write_fmt(format_args!($fmt $(,$($arg)+)?)).unwrap();
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(,$($arg: tt) +)?) => {
        $crate::console::Stdout.write_fmt(format_args!(concat!($fmt,"\n") $(,$($arg)+)?)).unwrap();
    }
}


