#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(trace_macros)]
#![feature(const_fn)]
#![allow(dead_code)]

mod lpae;
mod common;
mod frame_alloc;
mod esr;
mod memory_attrs;
mod uart;
mod aarch64;
mod vm;


use core::panic::PanicInfo;

mod irq;

#[cfg(not(feature="hypervisor_test"))]
mod start;
#[cfg(not(feature="hypervisor_test"))]
pub use start::start_hypervisor;

#[cfg(feature="hypervisor_test")]
mod test;

#[cfg(feature="hypervisor_test")]
pub use test::start_hypervisor;


#[panic_handler]
fn handler(_x: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_unwind_resume"]
extern "C" fn rust_eh_unwind_resume() {}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running {} tests", tests.len());
            for test in tests {
                        test();
                            }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
