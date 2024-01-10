pub fn putchar(c : usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(f : bool) -> ! {
    use sbi_rt::{Shutdown,SystemFailure,NoReason}; 
    if !f {
        sbi_rt::system_reset(Shutdown,NoReason);
    } else {
        sbi_rt::system_reset(Shutdown,SystemFailure);
    }
   loop {} 
}


