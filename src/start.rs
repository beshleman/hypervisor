use crate::lpae::{
    PageTableEntry,
    PageTable,
    Alignment,
    align,
    ADDRESS_SPACE_PER_TABLE,
    PAGE_SIZE,
    pagetable_zeroeth_index,
    pagetable_first_index,
    pagetable_second_index,
    pagetable_third_index
};

/* TODO: look into options for avoiding static mut */
static mut ZEROETH: PageTable = [PageTableEntry { pte: 0 }; 512];
static mut FIRST: PageTable = [PageTableEntry { pte: 0 }; 512];
static mut SECOND: PageTable = [PageTableEntry { pte: 0 }; 512];
static mut THIRD: PageTable = [PageTableEntry { pte: 0 }; 512];

fn map_address_range(virt_start: u64, virt_end: u64, phys_start: u64) -> () {
    let start = align(virt_start, Alignment::Kb4);
    let end = align(virt_end, Alignment::Kb4);
    let mut paddr = phys_start;

    for vaddr in (start..end).step_by(PAGE_SIZE) {
        let index0 = pagetable_zeroeth_index(vaddr);
        let index1 = pagetable_first_index(vaddr);
        let index2 = pagetable_second_index(vaddr);
        let index3 = pagetable_third_index(vaddr);

        unsafe {
            ZEROETH[index0] = PageTableEntry::from_table(&FIRST);
            FIRST[index1] = PageTableEntry::from_table(&SECOND);
            SECOND[index2] = PageTableEntry::from_table(&THIRD);
            THIRD[index3] = PageTableEntry::from_block(paddr);
        }
            
        paddr += PAGE_SIZE as u64;
    }
}

fn setup_boot_pagetables(s: u64, e: u64, _offset: u64) -> ! {
    /*
     * This pagetable code assumes that we can fit the entire hypervisor
     * into a single stage1 table mapping, which is 2MB.
     *
     * Panic if the hypervisor is larger than 2MB.
     */
    assert!(!(e - s > (ADDRESS_SPACE_PER_TABLE as u64)));

    /* Identity map the hypervisor (virtual address == physical address) */
    map_address_range(s, e, s);

    /* TODO: map RO, .text, .bss, etc... separately w/ appropriate permissions */
    loop {}
}

#[no_mangle]
pub fn start_mythril(start: u64, end: u64, offset: u64) -> () {
    setup_boot_pagetables(start, end, offset);
}
