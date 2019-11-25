use crate::uart::uart_write;
use crate::esr::print_exception_syndrome;
use crate::aarch64::{current_el, ExceptionLevel};
use crate::mrs;
use crate::common::print_hex;

fn print_spsr_el2() -> () {

    let mut spsr: u64;

    mrs!(spsr, "SPSR_EL2");
    let m: u64 = spsr & 0xf;

    uart_write("EL from SPSR.M[4:0] : ");
    uart_write("0b");

    for _ in 0..4 {
        if ((spsr & (1<<3)) >> 3) == 1 {
            uart_write("1");
        } else {
            uart_write("0");
        }

        spsr <<= 1;
    }
    uart_write("\n");
    uart_write("EL from SPSR.M[4:0] : ");
    match m {
        0b0000 => uart_write("EL0t"),
        0b0100 => uart_write("EL1t"),
        0b0101 => uart_write("EL1h"),
        0b1000 => uart_write("EL2t"),
        0b1001 => uart_write("EL2h"),
            _ => uart_write("Unknown"),
    }
    uart_write("\n");
}

fn print_current_el() -> () {
    uart_write("Current EL: ");
    match current_el() {
        0 => uart_write("EL0"),
        1 => uart_write("EL1"),
        2 => uart_write("EL2"),
        3 => uart_write("EL3"),
        _ => loop{},
    }
    uart_write("\n");
}


fn print_elr_el2() -> () {
    let elr_el2: u64;

    uart_write("ELR_EL2: ");
    mrs!(elr_el2, "ELR_EL2");

    print_hex(elr_el2);
    uart_write("\n");
}

//ELR_EL2        0x40400000


#[no_mangle]
pub extern fn irq_handler() -> ! {
    print_current_el();
    print_spsr_el2();
    print_elr_el2();
    print_exception_syndrome(ExceptionLevel::EL1);
    print_exception_syndrome(ExceptionLevel::EL2);
    loop {}
}

