
static mut UART_VIRT: u64 = 0;

pub fn uart_init(uart_virt: u64) -> () {
    unsafe {
        UART_VIRT = uart_virt
    };
}

pub fn uart_write(string: &str) -> () {
    unsafe {
        if UART_VIRT == 0 {
            return;
        }
    }

    let p = unsafe { 
        UART_VIRT as *mut u64
    };

    let bytes = string.as_bytes();
    for byte in bytes {
        unsafe { *p = *byte as u64; }
    }
}

