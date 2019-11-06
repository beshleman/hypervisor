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

