use crate::lpae::{
    PageTableEntry,
    PageTable,
    VirtualAddress,
    Alignment,
    PAGE_SIZE,
    pagetable_zeroeth_index,
    pagetable_first_index,
    pagetable_second_index,
    pagetable_third_index
};

/* TODO: look into avoiding static mut */
static mut ZEROETH: PageTable = [PageTableEntry { pte: 0 }; 512];
#[allow(dead_code)]
static mut FIRST: PageTable = [PageTableEntry { pte: 0 }; 512];
#[allow(dead_code)]
static mut SECOND: PageTable = [PageTableEntry { pte: 0 }; 512];
#[allow(dead_code)]
static mut THIRD: PageTable = [PageTableEntry { pte: 0 }; 512];

fn setup_boot_pagetables(s: u64, e: u64, _offset: u64) -> ! {
    let start = VirtualAddress::new(s).aligned(Alignment::Kb4);
    let end = VirtualAddress::new(e).aligned(Alignment::Kb4);

    /* From _start to _end, create an identity map where the virtual address is identical to the
     * physical address address.
     */
    for page in (start.vaddr()..end.vaddr()).step_by(PAGE_SIZE as usize) {
        let vaddr = VirtualAddress::new(page);

        unsafe {
            ZEROETH[pagetable_zeroeth_index(&vaddr)] =
                PageTableEntry::from_table(&FIRST);

            FIRST[pagetable_first_index(&vaddr)] =
                PageTableEntry::from_table(&SECOND);

            SECOND[pagetable_second_index(&vaddr)] =
                PageTableEntry::from_table(&THIRD);

            THIRD[pagetable_third_index(&vaddr)] =
                PageTableEntry::from_block(page);
        }
    }

    /* TODO */
    /* From _start to _end, create an mapping where the virtual address is equal
     * the compile-time address.
     */

    // TODO: protect .text as read only
    // TODO: protect other memory ranges
    //
/*        ZEROETH[0] = PageTableEntry { pte: 0 };
    unsafe {
        FIRST[0] = PageTableEntry { pte: 0 };
        SECOND[0] = PageTableEntry { pte: 0 };
        THIRD[0] = PageTableEntry { pte: 0 };
    }
    */

    loop {}
}

#[no_mangle]
pub fn start_mythril(start: u64, end: u64, offset: u64) -> () {
    setup_boot_pagetables(start, end, offset);
}
