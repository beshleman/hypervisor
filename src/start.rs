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

use crate::esr::print_exception_syndrome;
use crate::{msr, mrs};
use crate::common::bit;
use crate::memory_attrs;
use crate::uart::{uart_write, uart_init};
use crate::aarch64::{current_el, Shareable, data_barrier, isb, ExceptionLevel};

const UART_BASE: u64 = 0x09000000;
const UART_SIZE: u64 = 0x00001000;

struct PageTableTree {
    zeroeth: PageTable,
    first: PageTable,
    second: PageTable,
    third: PageTable,
}

impl PageTableTree {
    fn new() -> PageTableTree {
        PageTableTree {
            zeroeth: PageTable::new(),
            first: PageTable::new(),
            second: PageTable::new(),
            third: PageTable::new(),
        }
    }

    fn map(&mut self, vaddr: u64, paddr: u64) -> () {
        let index0 = pagetable_zeroeth_index(vaddr);
        let index1 = pagetable_first_index(vaddr);
        let index2 = pagetable_second_index(vaddr);
        let index3 = pagetable_third_index(vaddr);

        self.zeroeth.entries[index0] = PageTableEntry::from_table(&self.first);
        self.first.entries[index1] = PageTableEntry::from_table(&self.second);
        self.second.entries[index2] = PageTableEntry::from_table(&self.third);
        self.third.entries[index3] = PageTableEntry::from_block(paddr);
    }
}

const SCTLR_EL2_RES1: u64 = (bit(4) | bit(5) | bit(11) |
                             bit(16) | bit(18) | bit(22) |
                             bit(23) | bit(28) | bit(29));

const HCR_EL2_E2H_SHIFT: u64 = 34;
const TCR_EL2_INNER_WRITE_BACK_WRITE_ALLOC: u64 = bit(8);
const TCR_EL2_OUTER_WRITE_BACK_WRITE_ALLOC: u64 = bit(10);
const TCR_EL2_INNER_SHAREABLE: u64 = bit(12) | bit(13);
const TCR_EL2_RES1: u64 = bit(31) | bit(23);

fn map_address_range(boot_table_tree: &mut PageTableTree,
                     virt_start: u64,
                     virt_end: u64,
                     phys_start: u64) -> () {

    let start = align(virt_start, Alignment::Kb4);
    let end = align(virt_end, Alignment::Kb4);
    let mut paddr = align(phys_start, Alignment::Kb4);

    for vaddr in (start..end).step_by(PAGE_SIZE) {
        boot_table_tree.map(vaddr, paddr);
        paddr += PAGE_SIZE as u64;
    }
}

fn setup_boot_pagetables(boot_table_tree: &mut PageTableTree,
                         start: u64,
                         end: u64,
                         offset: u64) -> () {
    /*
     * This pagetable code assumes that we can fit the entire hypervisor
     * into a single stage1, 4-level table mapping, which is 2MB.
     *
     * Panic if the hypervisor is larger than 2MB.
     */
    assert!(!(end - start > (ADDRESS_SPACE_PER_TABLE as u64)));

    /* The offset MUST be exactly 1GB from identity map, this way the
     * level one entries for the identity map and the offset map both
     * point to the same level two entry, which then maps down to the same level three
     * and so forth...
     */
    assert!((offset % (1 << 30)) == 0);

    /* Identity map the hypervisor (virtual address == physical address) */
    map_address_range(boot_table_tree, start, end, start);

    /* Map the hypervisor load address space to its real physical address space */
    map_address_range(boot_table_tree, start + offset,
                      end + offset, start);

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

const HCR_VM: u64 = bit(0);
const HCR_IMO: u64 = bit(3);
const HCR_FMO: u64 = bit(4);
const HCR_AMO: u64 = bit(5);
const HCR_RW: u64 = bit(31);
const HCR_TGE: u64 = bit(27);

fn enable_virt() -> () {
    let mut hcr_el2: u64;

    mrs!(hcr_el2, "HCR_EL2");
    hcr_el2 |= HCR_VM;
    msr!("HCR_EL2", hcr_el2 | HCR_VM);
}

pub fn trap_lower_el_into_el2() -> () {
    /* Can't use msr!() because DAIFSet only takes immedates */
    unsafe {
        asm!("msr DAIFSet, 0x0");
    }

    let mut hcr_el2: u64;
    mrs!(hcr_el2, "HCR_EL2");
    hcr_el2 |= HCR_AMO | HCR_FMO | HCR_IMO;

    /*
    /* Trap General Exceptions into EL2 (instead of EL1) */
    hcr_el2 |= HCR_TGE;
    */

    /* Lower exception levels are Aarch64 */
    hcr_el2 |=  HCR_RW;

    msr!("HCR_EL2", hcr_el2);
}

fn flush_hypervisor_tlb() -> () {
    unsafe { asm!("tlbi alle2") };
    data_barrier(Shareable::Non);
}

fn switch_ttbr(pagetable_address: u64) -> () {
    msr!("TTBR0_EL2", pagetable_address);
    isb();
}

fn switch_vttbr(pagetable_address: u64) -> () {
    msr!("vttbr_el2", pagetable_address);
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
    msr!("hcr_el2", hcr_el2);
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

const VTCR_EL2_SL0: u64 = bit(7);

fn init_vtcr() -> () {
    let mut vctr_el2: u64 = 0;

    vctr_el2 |= TCR_EL2_RES1;
    vctr_el2 |= TCR_EL2_INNER_WRITE_BACK_WRITE_ALLOC;
    vctr_el2 |= TCR_EL2_OUTER_WRITE_BACK_WRITE_ALLOC;
    vctr_el2 |= TCR_EL2_INNER_SHAREABLE;
    vctr_el2 |= VTCR_EL2_SL0;

    // TCR_EL2.PS[18:16]
    let pa_range = get_phys_addr_range();
    vctr_el2 |= (pa_range & 0x3) << 16;

    // 48-bit virtual address space
    vctr_el2 |= 64 - 48;
    msr!("vtcr_el2", vctr_el2);
}

fn enable_mmu() -> () { 
    let mut sctlr_el2: u64;

    mrs!(sctlr_el2, "sctlr_el2");

    /* Enable the MMU */
    sctlr_el2 |= bit(0);

    /* Sync all pagetable modifications */
    data_barrier(Shareable::FullSystem);

    msr!("sctlr_el2", sctlr_el2);
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

fn print_spsr_el2() -> () {

    let mut spsr: u64;

    mrs!(spsr, "SPSR_EL2");
    let m: u64 = spsr & 0xf;

    uart_write("EL from SPSR.M[4:0] : ");
    uart_write("0b");

    for _ in 0..4 {
        if ((spsr & (1<<3)) >> 3) == 1 {
            uart_write("1");
        } else {
            uart_write("0");
        }

        spsr <<= 1;
    }
    uart_write("\n");
    uart_write("EL from SPSR.M[4:0] : ");
    match m {
        0b0000 => uart_write("EL0t"),
        0b0100 => uart_write("EL1t"),
        0b0101 => uart_write("EL1h"),
        0b1000 => uart_write("EL2t"),
        0b1001 => uart_write("EL2h"),
            _ => uart_write("Unknown"),
    }
    uart_write("\n");
}

fn print_current_el() -> () {
    uart_write("Current EL: ");
    match current_el() {
        0 => uart_write("EL0"),
        1 => uart_write("EL1"),
        2 => uart_write("EL2"),
        3 => uart_write("EL3"),
        _ => loop{},
    }
    uart_write("\n");
}

#[no_mangle]
pub extern fn irq_handler() -> ! {
    print_current_el();
    print_spsr_el2();
    print_exception_syndrome(ExceptionLevel::EL1);
    print_exception_syndrome(ExceptionLevel::EL2);
    loop {}
}

fn init_el2_interrupts(irq_vector_addr: u64) -> () {
    let vbar_el2 = irq_vector_addr;

    // If the implementation does not support ARMv8.2-LVA, then
    // bits VBAR_EL2[63:48] must be zero.
    match vbar_el2 & !((1<<48) - 1) {
        0 => {
            msr!("vbar_el2", vbar_el2);
            isb();
            trap_lower_el_into_el2();
        },
        _ => loop {}
    }

}

fn init_el1_interrupts(irq_vector_addr: u64) -> () {
    let vbar_el2 = irq_vector_addr;

    // If the implementation does not support ARMv8.2-LVA, then
    // bits VBAR_EL2[63:48] must be zero.
    match vbar_el2 & !((1<<48) - 1) {
        0 => {
            msr!("vbar_el2", vbar_el2);
            isb();
            trap_lower_el_into_el2();
        },
        _ => loop {}
    }

}


/*
 * 0b0000 EL0t.
 * 0b0100 EL1t.
 * 0b0101 EL1h.
 * 0b1000 EL2t.
 * 0b1001 EL2h.
*/

#[allow(non_upper_case_globals)]
const SPSR_EL0t: u64 = 0b0000;
#[allow(non_upper_case_globals)]
const SPSR_EL1t: u64 = 0b0100;
#[allow(non_upper_case_globals)]
const SPSR_EL1h: u64 = 0b0101;
#[allow(non_upper_case_globals)]
const SPSR_EL2t: u64 = 0b1000;
#[allow(non_upper_case_globals)]
const SPSR_EL2h: u64 = 0b1001;

pub fn load_guest() -> () {
    let guest_address: u64 = 0x40400000;

    let mut stage2_table = PageTableTree::new();
    stage2_table.map(guest_address, guest_address);

    /* Initialize VTCR_EL2 */
    init_vtcr();

    /* Initialize VTTBR_EL2 */
    let vttbr_el2 = &stage2_table as *const _ as u64;
    switch_vttbr(vttbr_el2);
    
    unsafe { asm!("msr SCTLR_EL1, XZR"); }
    
     /* Mask interrupts */ 
    unsafe { asm!("msr daifclr, #0xf"); }

    /*
     * To return from an exception, use the ERET instruction. This instruction restores
     * processor state
     * by copying SPSR_ELn to PSTATE and branches to the saved return address in ELR_ELn.
     */

    msr!("ELR_EL2", guest_address);

    //const EL0t: u64 = 0b0000;
    //const EL1h: u64 = 0b0101;
    //let spsr_el2: u64 = EL1h;
    //let spsr_el2: u64 = EL0t;
    // 0b0000 => EL0 (sync_lower_64)
    // 0b0001 => EL2h
    // 0b0010 => EL2h
    // 0b0011 => EL2h
    // 0b0100 => EL0 (sync_lower_64)
    // 0b0101 => EL0
    // 0b0110 => EL2h
    // 0b0111 => EL2h
    // 0b1000 => EL2t
    // 0b1001 => EL2h
    // 0b1010 => EL2h
    // 0b1101 => EL2h
    // 0b1111 => EL2h
    msr!("SPSR_EL2", SPSR_EL2t);

    //data_abort();

    unsafe {
        asm!("eret");
    }
}

#[allow(dead_code)]
fn data_abort() -> () {
    // cause interrupt
    unsafe {
        let p = 0 as *const i32;
        let mut _k = *p;
    }
}

#[no_mangle]
pub extern fn start_hypervisor(start: u64,
                               end: u64,
                               offset: u64,
                               irq_vector_addr: u64) -> ! {
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

    let mut boot_table_tree = PageTableTree::new();
    setup_boot_pagetables(&mut boot_table_tree, start, end, offset);

    let uart_virt = align(end + PAGE_SIZE as u64, Alignment::Kb4);
    boot_table_tree.map(uart_virt, UART_BASE);
    //let guest_address: u64 = 0x40400000;
    //boot_table_tree.map(guest_address, guest_address);

    /* Flush the tlb just in case there is stale state */
    flush_hypervisor_tlb();

    let ttbr0_el2 = &boot_table_tree.zeroeth as *const _ as u64;
    switch_ttbr(ttbr0_el2);
    enable_mmu();
    init_el2_interrupts(irq_vector_addr);
    init_el1_interrupts(irq_vector_addr);

    uart_init(uart_virt);
    uart_write("UART mapped\n");


    enable_virt();
    load_guest();

    loop {}
}
