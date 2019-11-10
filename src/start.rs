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
use crate::common::bit;
use crate::memory_attrs;
use crate::aarch64::{current_el, Shareable, data_barrier, isb};

const SCTLR_EL2_RES1: u64 = (bit(4) | bit(5) | bit(11) |
                             bit(16) | bit(18) | bit(22) |
                             bit(23) | bit(28) | bit(29));

const HCR_EL2_E2H_SHIFT: u64 = 34;
const TCR_EL2_INNER_WRITE_BACK_WRITE_ALLOC: u64 = bit(8);
const TCR_EL2_OUTER_WRITE_BACK_WRITE_ALLOC: u64 = bit(10);
const TCR_EL2_INNER_SHAREABLE: u64 = bit(12) | bit(13);
const TCR_EL2_RES1: u64 = bit(31) | bit(23);

fn map_address_range(zeroeth: &mut PageTable,
                     first:   &mut PageTable,
                     second:  &mut PageTable,
                     third:   &mut PageTable,
                     virt_start: u64,
                     virt_end: u64,
                     phys_start: u64) -> () {

    let start = align(virt_start, Alignment::Kb4);
    let end = align(virt_end, Alignment::Kb4);
    let mut paddr = align(phys_start, Alignment::Kb4);

    for vaddr in (start..end).step_by(PAGE_SIZE) {
        let index0 = pagetable_zeroeth_index(vaddr);
        let index1 = pagetable_first_index(vaddr);
        let index2 = pagetable_second_index(vaddr);
        let index3 = pagetable_third_index(vaddr);

        zeroeth[index0] = PageTableEntry::from_table(&first);
        first[index1] = PageTableEntry::from_table(&second);
        second[index2] = PageTableEntry::from_table(&third);
        third[index3] = PageTableEntry::from_block(paddr);
            
        paddr += PAGE_SIZE as u64;
    }
}

fn setup_boot_pagetables(zeroeth: &mut PageTable,
                         first:   &mut PageTable,
                         second:  &mut PageTable,
                         third:   &mut PageTable,
                         start: u64,
                         end: u64,
                         _offset: u64) -> () {
    /*
     * This pagetable code assumes that we can fit the entire hypervisor
     * into a single stage1, 4-level table mapping, which is 2MB.
     *
     * Panic if the hypervisor is larger than 2MB.
     */
    assert!(!(end - start > (ADDRESS_SPACE_PER_TABLE as u64)));

    /* Identity map the hypervisor (virtual address == physical address) */
    map_address_range(zeroeth,
                      first,
                      second,
                      third,
                      start, end, start);

    /* Map the hypervisor load address space to its real physical address space */
    map_address_range(zeroeth,
                      first,
                      second,
                      third,
                      start + _offset,
                      end + _offset, start);

    /* TODO: map RO, .text, .bss, etc... separately w/ appropriate permissions */

    /* Set Memory Attribute Indirect Register (MAIR) */
    memory_attrs::init();
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

fn switch_ttbr(pagetable_address: u64) -> () {
    msr!("TTBR0_EL2", pagetable_address);
    isb();
}

fn get_phys_addr_range() -> u64 {
    let mmfr0: u64;

    mrs!(mmfr0, "ID_AA64MMFR0_EL1"); 

    // ID_AA64MMFR0_EL1[3:0] is PARange
    mmfr0 & 0xf
}

fn disable_el2_host() -> () {
    // D13.2.46 HCR_EL2, Hypervisor Configuration Register
    let mut hcr_el2: u64;
    mrs!(hcr_el2, "HCR_EL2");
    hcr_el2 &= !(1 << HCR_EL2_E2H_SHIFT);
    msr!("HCR_EL2", hcr_el2);
}

fn init_tcr() -> () {
    let mut tcr_el2: u64 = 0;

    tcr_el2 |= TCR_EL2_RES1;
    tcr_el2 |= TCR_EL2_INNER_WRITE_BACK_WRITE_ALLOC;
    tcr_el2 |= TCR_EL2_OUTER_WRITE_BACK_WRITE_ALLOC;
    tcr_el2 |= TCR_EL2_INNER_SHAREABLE;

    // TCR_EL2.PS[18:16]
    let pa_range = get_phys_addr_range();
    tcr_el2 |= (pa_range & 0x3) << 16;

    // 48-bit virtual address space
    tcr_el2 |= 64 - 48;
    msr!("tcr_el2", tcr_el2);
}

fn enable_mmu() -> () { 
    let mut sctlr_el2: u64;

    mrs!(sctlr_el2, "SCTLR_EL2");

    /* Enable the MMU */
    sctlr_el2 |= bit(1);

    /* Sync all pagetable modifications */
    data_barrier(Shareable::FullSystem);

    msr!("SCTLR_EL2", sctlr_el2);
    isb();
}

fn init_sctlr() -> () {
    /* Default to zero so we don't have to set all of the RES0 bits */
    let mut sctlr_el2: u64 = 0;

    /* Set all RES1 bits */
    sctlr_el2 |= SCTLR_EL2_RES1;

    /* Enable the D-cache */
    sctlr_el2 |= bit(2);

    /*
     * Cause SP Alignment fault if the stack pointer is used in a load/store
     * but is not 16 byte aligned
     */
    sctlr_el2 |= bit(3);

    /*
     * If the MMU is on (SCTLR_EL2.M == 1), then instruction accesses from
     * stage 1 of the EL2 translation regime are to Normal, Outer Shareable,
     * Inner Write-Through, Outer Write-Through memory.
     */
    sctlr_el2 |= bit(12);

    msr!("SCTLR_EL2", sctlr_el2);

    /* Make sure SCTLR_EL2 is loaded before we continue */
    isb();
}

#[no_mangle]
pub extern fn start_hypervisor(start: u64, end: u64, offset: u64) -> ! {
    assert_eq!(current_el(), 2);
    disable_interrupts();

    /*
     * As a baremetal hypervisor with no nested hypervisor support
     * we do not need to support hosts at EL2.
     */
    disable_el2_host();

    init_tcr();
    init_sctlr();
    unsafe { asm!("msr spsel, #1") }


    /* TODO: look into options for avoiding static mut */
    /* TODO: Use borrow checker to maintain single references (singleton) */
    /* TODO: zero these tables */
    let mut zeroeth: PageTable = [PageTableEntry(0); 512];
    let mut first: PageTable = [PageTableEntry(0); 512];
    let mut second: PageTable = [PageTableEntry(0); 512];
    let mut third: PageTable = [PageTableEntry(0); 512];

    setup_boot_pagetables(&mut zeroeth, &mut first, &mut second, &mut third, start, end, offset);

    /* Flush the tlb just in case there is stale state */
    flush_hypervisor_tlb();

    let ttbr0_el2 = &zeroeth as *const _ as u64;
    switch_ttbr(ttbr0_el2);
    enable_mmu();

    loop {}
}
