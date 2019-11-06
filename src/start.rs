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

use crate::{msr, mrs};
use crate::memory_attrs;
use crate::aarch64::{current_el, Shareable, data_barrier};

/* TODO: look into options for avoiding static mut */
/* TODO: Use borrow checker to maintain single references (singleton) */
static mut ZEROETH: PageTable = [PageTableEntry { pte: 0 }; 512];
static mut FIRST: PageTable = [PageTableEntry { pte: 0 }; 512];
static mut SECOND: PageTable = [PageTableEntry { pte: 0 }; 512];
static mut THIRD: PageTable = [PageTableEntry { pte: 0 }; 512];

fn map_address_range(virt_start: u64, virt_end: u64, phys_start: u64) -> () {
    let start = align(virt_start, Alignment::Kb4);
    let end = align(virt_end, Alignment::Kb4);
    let mut paddr = align(phys_start, Alignment::Kb4);

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

fn setup_boot_pagetables(start: u64, end: u64, _offset: u64) -> ! {
    /*
     * This pagetable code assumes that we can fit the entire hypervisor
     * into a single stage1 table mapping, which is 2MB.
     *
     * Panic if the hypervisor is larger than 2MB.
     */
    assert!(!(end - start > (ADDRESS_SPACE_PER_TABLE as u64)));

    /* TODO: panic if we are not in EL2 */

    /* Identity map the hypervisor (virtual address == physical address) */
    map_address_range(start, end, start);

    /* Map the hypervisor load address space to its real physical address space */
    map_address_range(start + _offset, end + _offset, start);

    /* TODO: map RO, .text, .bss, etc... separately w/ appropriate permissions */

    /* Set Memory Attribute Indirect Register (MAIR) */
    memory_attrs::init();

    /* TODO: Initialize processor for enabling the MMU */
    /* TODO: Enable the MMU */

    loop {}
}

pub fn disable_interrupts() -> () {
    /* Can't use msr!() because DAIFSet only takes immedates */
    unsafe {
        asm!("msr DAIFSet, 0xf");
    }
}

fn flush_hypervisor_tlb() -> () {
    unsafe { asm!("tlbi alle2") };
    data_barrier(Shareable::Non);
}

fn isb() ->() {
    unsafe { asm!("isb"); }
}

fn switch_ttbr(pagetable: &PageTable) -> () {
    let ttbr0_el2 = &pagetable as *const _ as u64;
    msr!("TTBR0_EL2", ttbr0_el2);
    isb();
}

fn enable_mmu() -> () {
    let mut sctlr_el2: u64;

    /* Flush the tlb just in case there is stale state */
    flush_hypervisor_tlb();

    unsafe {
        switch_ttbr(&ZEROETH);
    }

    mrs!(sctlr_el2, "SCTLR_EL2");

    /* Enable the MMU */
    sctlr_el2 |= 1;

    /* Enable the D-cache */
    sctlr_el2 |= 1 << 2;

    /* Sync all pagetable modifications */
    data_barrier(Shareable::FullSystem);

    msr!("SCTLR_EL2", sctlr_el2);
    unsafe{ asm!("isb") }

    

    
    /*
     *  /*DONE*/
        tlbi  alle2                  /* Flush hypervisor TLBs */
        dsb   nsh

        /* Write Xen's PT's paddr into TTBR0_EL2 */
        load_paddr x0, boot_pgtable
        msr   TTBR0_EL2, x0
        isb

        mrs   x0, SCTLR_EL2
        orr   x0, x0, #SCTLR_Axx_ELx_M  /* Enable MMU */
        orr   x0, x0, #SCTLR_Axx_ELx_C  /* Enable D-cache */
        dsb   sy                     /* Flush PTE writes and finish reads */
        msr   SCTLR_EL2, x0          /* now paging is enabled */
        isb                          /* Now, flush the icache */
        */

}

#[no_mangle]
pub fn start_mythril(start: u64, end: u64, offset: u64) -> () {
    let el = current_el();
    assert_eq!(el, 2);

    /* TMP: test that turning on the mmu w/out page tables kills the program */
    enable_mmu();

    disable_interrupts();
    setup_boot_pagetables(start, end, offset);
}
