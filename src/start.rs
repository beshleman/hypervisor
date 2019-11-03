type LpaeEntry = u64;

/* TODO: consider changing this to be lock protected once SMP is supported */
static mut ROOT_PAGETABLE : [LpaeEntry; 512] = [0; 512];

fn setup_boot_pagetables(_start: u64, _end: u64, _offset: u64) -> () {
    /* From _start to _end, create an identity map where the virtual address is identical to the
     * physical address address.
     */

    /* TODO */

    /* From _start to _end, create an mappning where the virtual address is equal
     * compile-time address.
     */

    // TODO: protect .text as read only
    // TODO: protect other memory ranges
    //
    unsafe { ROOT_PAGETABLE[0] = 0; }
}

#[no_mangle]
pub fn start_mythril(start: u64, end: u64, offset: u64) -> () {
    setup_boot_pagetables(start, end, offset);
    loop {}
}
