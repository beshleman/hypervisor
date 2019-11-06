#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(asm)]
#![feature(trace_macros)]

mod start;
mod lpae;
mod memory_attrs;
mod aarch64;

pub use start::start_mythril;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn handler(_x: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_unwind_resume"]
extern "C" fn rust_eh_unwind_resume() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
