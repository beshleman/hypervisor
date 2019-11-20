#[cfg(feature="hypervisor_test")]
#[no_mangle]
pub extern fn start_hypervisor(_start: u64,
                               _end: u64,
                               _offset: u64,
                               _irq_vector_addr: u64) -> ! {


    loop {}
}
