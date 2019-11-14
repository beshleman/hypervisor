#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(asm)]
#![feature(trace_macros)]
#![feature(const_fn)]
#![allow(dead_code)]

mod start;
mod lpae;
mod memory_attrs;
mod aarch64;
mod common;
mod frame_alloc;
mod esr;
mod uart;

pub use start::start_hypervisor;

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
