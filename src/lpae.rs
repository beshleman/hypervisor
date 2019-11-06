/*
 * This module implements the Large Physical Address Extension (LPAE)
 * for ARM.
 *
 *
 * Please refer to: "The AArch64 Virtual Memory System Architecture D5.3
 * VMSAv8-64 translation table format descriptors" in the manual.
 */
#![allow(dead_code)]

pub type VirtualAddress = u64;
pub type PageTable = [PageTableEntry; 512];

const PAGE_SHIFT: u64 = 12;
const PAGE_MASK: u64 = (1 << PAGE_SHIFT) - 1;
pub const PAGE_SIZE: usize = (1 << PAGE_SHIFT) as usize;

const PAGE_ENTRIES_PER_TABLE: usize = 512;
pub const ADDRESS_SPACE_PER_TABLE: usize = PAGE_SIZE * PAGE_ENTRIES_PER_TABLE;

const PTE_SHIFT_4K: u64 = 9;
const PTE_SHIFT_16K: u64 = 1;
const PTE_SHIFT_64K: u64 = 13;
const PTE_SHIFT: u64 = 9;
const PTE_MASK: u64 = (1 << PTE_SHIFT) - 1;

const THIRD_SHIFT: u64 =  PAGE_SHIFT;
const THIRD_SIZE: u64 =   (THIRD_SHIFT);
const THIRD_MASK: u64 =   (!(THIRD_SIZE - 1));

const SECOND_SHIFT: u64 = (THIRD_SHIFT + PTE_SHIFT);
const SECOND_SIZE: u64 =  (1 << SECOND_SHIFT);
const SECOND_MASK: u64 =  (!(SECOND_SIZE - 1));

const FIRST_SHIFT: u64 =  (SECOND_SHIFT + PTE_SHIFT);
const FIRST_SIZE: u64 =   (1 << FIRST_SHIFT);
const FIRST_MASK: u64 =   (!(FIRST_SIZE - 1));

const ZEROETH_SHIFT: u64 = (FIRST_SHIFT + PTE_SHIFT);
const ZEROETH_SIZE: u64 =  (1 << ZEROETH_SHIFT);
const ZEROETH_MASK: u64 =  (!(ZEROETH_SIZE - 1));

const ALIGN_4K_MASK: u64 = !PAGE_MASK;

pub enum Alignment {
    Kb4,
}

impl Alignment {
    fn mask(self) -> u64 {
        match self {
            Alignment::Kb4 => ALIGN_4K_MASK,
        }
    }
}

/* ARM LPAE entries are 64-bit */
#[derive(Copy, Clone, Debug)]
pub struct PageTableEntry {
    pub pte: u64,
}

/**
 * The LPAE page table entry bit layout is as follows:
 *  format: field name       : starting bit, size (in bits)
 *  
 *  (Used by both block and table entries)
 *  Valid                    : 0, 1
 *  Table                    : 1, 1
 *
 *  (Used only for Block Entries, ignored in Table Entries)
 *  Attribute Index          : 2, 3
 *  Non-Secure               : 5, 1
 *  Unprivileged Access      : 6, 1
 *  Read-Only                : 7, 1
 *  Shareability             : 8, 2
 *  Access Flag              : 10, 1
 *  Not-Global               : 11, 1
 *
 *  (Used by both block and table entries)
 *  Block / table address    : 12, 36
 *  must be zero             : 48, 4
 *
 *  (Used only for Block Entries, ignored in Table Entries)
 *  is a continiguous entry  : 52, 1
 *  privileged execute never : 53, 1
 *  execute never            : 54, 1
 *  ignored by hardware      : 55, 4
 *
 *  (Used only for Table entries, ignored in Block Entries)
 *  Privileged Execute Never : 59, 1
 *  Execute Never            : 60, 1
 *  Access Permissions       : 61, 2
 *  Not Secure               : 63, 1
 */

/* Valid: bit 0 */
const PTE_VALID_SHIFT: u64 = 0;
const PTE_VALID_BITS: u64 = 1;
const PTE_VALID_MASK: u64 = PTE_VALID_BITS << PTE_VALID_SHIFT;

/* Table: bit 1 */
const PTE_TABLE_SHIFT: u64 = 1;
const PTE_TABLE_BITS: u64 = 1;
const PTE_TABLE_MASK: u64 = PTE_TABLE_BITS << PTE_TABLE_SHIFT;

/* Attribute: bits 2..4 */
const PTE_ATTR_SHIFT: u64  = 2;
const PTE_ATTR_BITS: u64  = 3;
const PTE_ATTR_UNSHIFTED_MASK: u64 = (1 << PTE_ATTR_BITS) - 1;
const PTE_ATTR_MASK: u64  = PTE_ATTR_UNSHIFTED_MASK << PTE_ATTR_SHIFT;

/* Shareability attributes */
/* This makes the page entry SMP coherent */
const PTE_ATTR_INNER_SHARE_MASK: u64 = 0x3 << PTE_ATTR_SHIFT;

const PTE_NON_SECURE_SHIFT: u64  = 5;
const PTE_NON_SECURE_BITS: u64   = 1;
const PTE_NON_SECURE_UNSHIFTED_MASK: u64   = (1 << PTE_NON_SECURE_BITS) - 1;
const PTE_NON_SECURE_MASK: u64   =  PTE_ATTR_UNSHIFTED_MASK << PTE_ATTR_SHIFT;

const PTE_UNPRIVILIGED_ACCESS_SHIFT: u64  = 6;
const PTE_UNPRIVILIGED_ACCESS_BITS: u64  = 1;
const PTE_UNPRIVILIGED_ACCESS_UNSHIFTED_MASK: u64  =
                        (1 << PTE_UNPRIVILIGED_ACCESS_BITS) - 1;
const PTE_UNPRIVILIGED_ACCESS_MASK: u64 =
        PTE_UNPRIVILIGED_ACCESS_UNSHIFTED_MASK << PTE_UNPRIVILIGED_ACCESS_SHIFT;

const PTE_READ_ONLY_SHIFT: u64  = 7;
const PTE_SHAREABILITY_SHIFT: u64  = 8;
const PTE_ACCESS_FLAG_SHIFT: u64  = 10;
const PTE_NOT_GLOBAL_SHIFT: u64  = 11;

/* Common */
const PTE_ADDRESS_SHIFT: u64 = 12;
const PTE_ZERO_SHIFT: u64 = 48;

/* Block Entry Only */
const PTE_CONTIGUOUS_SHIFT: u64 = 52;
const PTE_PRIV_EX_NEVER_SHIFT: u64 = 53;
const PTE_EX_NEVER_SHIFT: u64 = 54;
const PTE_IGNORED_SHIFT: u64 = 55;

/* Used only by Table Entries */
const PTE_TABLE_PRIV_EX_NEVER_SHIFT: u64 = 59;
const PTE_TABLE_EX_NEVER_SHIFT: u64 = 60;
const PTE_TABLE_ACCESS_PERMS_SHIFT: u64 = 61;
const PTE_TABLE_NOT_SECURE_SHIFT: u64 = 63;

const PTE_VALID: u64 = 1;
const PTE_TABLE: u64 = 1 << PTE_TABLE_SHIFT;
const PTE_NOT_SECURE: u64 = 5;
const PTE_UNPRIVILEGED_ACCESS: u64 = 6;
const PTE_NOT_GLOBAL: u64 = 7;

/*
 *
 *
 *            
    .valid = 1,           /* Mappings are present */
    .table = 0,           /* Set to 1 for links and 4k maps */
    .ai = attr,
    .ns = 1,              /* Hyp mode is in the non-secure world */
    .up = 1,              /* See below */
    .ro = 0,              /* Assume read-write */
    .af = 1,              /* No need for access tracking */
    .ng = 1,              /* Makes TLB flushes easier */
    .contig = 0,          /* Assume non-contiguous */
    .xn = 1,              /* No need to execute outside .text */
    .avail = 0,

#define MT_DEVICE_nGnRnE 0x0
#define MT_NORMAL_NC     0x1
#define MT_NORMAL_WT     0x2
#define MT_NORMAL_WB     0x3
#define MT_DEVICE_nGnRE  0x4
#define MT_NORMAL        0x7
    */

impl PageTableEntry {
    pub fn new(pte: u64) -> PageTableEntry {
        return PageTableEntry { pte: pte };
    }

    pub fn from_table(table: &PageTable) -> PageTableEntry {
            let mut address = (table as *const PageTable) as u64;

            address <<= PTE_SHIFT;
            address |= PTE_VALID;
            address |= PTE_TABLE;
            address |= PTE_NON_SECURE_MASK;

            /* Hypervisor mappings are SMP coherent / inner shareable */
            address |= PTE_ATTR_INNER_SHARE_MASK;

            /* TODO: Set table bit */
            return PageTableEntry{ pte: address };
    }

    pub fn from_block(mut address: u64) -> PageTableEntry {
            address <<= PTE_SHIFT;

            /* For 4K mappings, PTE_TABLE is set too*/
            address |= PTE_TABLE;
            address |= PTE_VALID;

            /* TODO: Support other shareability attrs */
            /* Hypervisor mappings are SMP coherent / inner shareable */
            address |= PTE_ATTR_INNER_SHARE_MASK;

            /* TODO: Set block bit */
            return PageTableEntry{ pte: address << PTE_SHIFT };
    }
}

pub fn align(vaddr: VirtualAddress, alignment: Alignment) -> VirtualAddress {
    return vaddr & alignment.mask();
}

/* Helpers for navigating the page tables */
pub fn pagetable_zeroeth_index(vaddr: VirtualAddress) -> usize {
    return ((vaddr >> ZEROETH_SHIFT) & PTE_MASK) as usize;
}

pub fn pagetable_first_index(vaddr: VirtualAddress) -> usize {
    return ((vaddr >> FIRST_SHIFT) & PTE_MASK) as usize;
}

pub fn pagetable_second_index(vaddr: VirtualAddress) -> usize {
    return ((vaddr >> SECOND_SHIFT) & PTE_MASK) as usize;
}

pub fn pagetable_third_index(vaddr: VirtualAddress) -> usize {
    return ((vaddr >> THIRD_SHIFT) & PTE_MASK) as usize;
}

pub fn align_4k(addr: u64) -> u64 {
    return addr & ALIGN_4K_MASK;
}

pub fn zeroeth_index(addr: u64) -> usize {
    return ((addr >> ZEROETH_SHIFT) & PTE_MASK) as usize;
}
