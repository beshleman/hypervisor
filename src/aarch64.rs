#[macro_export]
macro_rules! msr {
    ($reg:expr, $op:expr) => {
        let __val = $op;
        unsafe {
            asm!(concat!("msr ", $reg, ", $0") :: "r"(__val))
        }
    }
}

#[macro_export]
macro_rules! mrs {
    ($var:expr, $spr:expr) => { 
        unsafe {
            asm!(concat!("mrs $0, ", $spr) : "=r"($var));
        }
    }
}

pub enum ExceptionLevel {
    EL0 = 0,
    EL1 = 1,
    EL2 = 2,
    EL3 = 3
}

pub fn current_el() -> u64 {
    let el: u64;

    mrs!(el, "CurrentEL");

    return el >> 2;
}

#[allow(dead_code)]
pub enum Shareable {
    Non,
    Inner,
    Outer,
    FullSystem,
}

pub fn data_barrier(sh: Shareable) -> () {
    match sh {
        Shareable::Non => unsafe { asm!("dsb nsh"); },
        Shareable::Inner => unsafe { asm!("dsb ish"); },
        Shareable::Outer => unsafe { asm!("dsb osh"); },
        Shareable::FullSystem => unsafe { asm!("dsb sy"); },
    }
}

pub fn isb() -> () {
    unsafe{ asm!("isb") }
}

