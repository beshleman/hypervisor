#![no_std]
#![no_main]
#![feature(lang_items)]

mod start;

pub use start::start_mythril;

use core::panic::PanicInfo;

static mut VAR: i32 = 0;

#[link_section = ".example_section"]
#[no_mangle]
fn myassert() {
    assert!(1 + 1 == 2);

    for _ in 1..10 {
        unsafe { VAR += 1; }
    }
}

#[panic_handler]
fn handler(_x: &PanicInfo) -> ! { loop{} }

#[lang = "eh_unwind_resume"] extern fn rust_eh_unwind_resume() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
