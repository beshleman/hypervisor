pub const fn bit(x: u64) -> u64 {
    1 << x
}

pub const fn bitfield(end: u64, start:u64) -> u64 {
    let size = end - start;
    let mask = (1 << size)  - 1;

    mask << start
}
