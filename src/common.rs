
use crate::uart::uart_write;

pub const fn bit(x: u64) -> u64 {
    1 << x
}

pub const fn bitfield(end: u64, start:u64) -> u64 {
    let size = end - start;
    let mask = (1 << size)  - 1;

    mask << start
}

pub fn to_hex(val: u64) -> &'static str {
    match val {
        0x0 => "0",
        0x1 => "1",
        0x2 => "2",
        0x3 => "3",
        0x4 => "4",
        0x5 => "5",
        0x6 => "6",
        0x7 => "7",
        0x8 => "8",
        0x9 => "9",
        0xa => "a",
        0xb => "b",
        0xc => "c",
        0xd => "d",
        0xe => "e",
        0xf => "f",
        _ => loop {},
    }
}

pub fn print_hex(val: u64) -> () {
    let mut shift = 60;
    let nibbles: usize = 64 / 4;

    uart_write("0x");
    for _ in 0..nibbles {
        uart_write(to_hex((val & (0xf << shift)) >> shift));
        shift -= 4;
    }
}
