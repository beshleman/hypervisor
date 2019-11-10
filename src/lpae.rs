/*
 * This module implements the Large Physical Address Extension (LPAE)
 * for ARM.
 *
 *
 * Please refer to: "The AArch64 Virtual Memory System Architecture D5.3
 * VMSAv8-64 translation table format descriptors" in the manual.
 */
#![allow(dead_code)]

use crate::common::{bit, bitfield};

pub type VirtualAddress = u64;

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
    
/// ARM 64-bit LPAE entries
/// Refer to the ARM Reference Manual, Figure D5-15 for 
/// the format of these block and table descriptors.
#[derive(Copy, Clone, Debug)]
pub struct PageTableEntry(pub u64);
pub type PageTable = [PageTableEntry; 512];

// The AP_TABLE_BITS is RES0 for EL2 w/ no ARMv8.1-VHE
const AP_TABLE_BITS: u64 = bitfield(62, 61);
const TABLE_DESCRIPTOR_RES0: u64 = bitfield(51, 48) | AP_TABLE_BITS;
const TABLE_NON_SECURE: u64 = bit(63);

// The maxium output address for the Cortex-A53
const CORTEX_A53_MAX_OA: u64 = (1 << 40) - 1;

impl PageTableEntry {
    pub fn from_table(table: &PageTable) -> PageTableEntry {
        let address: u64 = (table as *const PageTable) as u64;
        let mut descriptor: u64 = 0;

        // Set next level table address
        descriptor |=  (address >> 12) << 12;
        
        // This is hypervisor memory, so set the Non-Secure Table bit to 1
        descriptor |= TABLE_NON_SECURE;
        descriptor |= PTE_VALID;
        descriptor |= PTE_TABLE;

        assert_eq!(descriptor & TABLE_DESCRIPTOR_RES0, 0);
        return PageTableEntry(descriptor);
    }

    /// Currently we only support level-3, 4KB blocks
    ///
    /// The documentation for these blocks can be found
    /// in "Figure D5-17 VMSAv8-64 level 3 descriptor format"
    /// of the ARMv8 reference manual.
    pub fn from_block(address: u64) -> PageTableEntry {
        assert!(address < CORTEX_A53_MAX_OA);
        let mut descriptor = 0;

        descriptor |= address & !((1 << 12) - 1);

        /* For 4K mappings, PTE_TABLE is set too*/
        descriptor |= PTE_TABLE;
        descriptor |= PTE_VALID;

        // 0xf7f == nG=1 AF=1 SH=11 AP=01 NS=1 ATTR=111 T=1 P=1 */

        // Use memory attr 000, which is inner-shareable, WBWA
        descriptor &= !(bit(4) | bit(3) | bit(2));

        // This is a Non-Secure block, NS == 1
        descriptor |= bit(5);

        // Access Permissions == EL2 Read/Write, no EL1 or EL0 access
        // (i.e., AP[1:0]  == 00)
        descriptor &= !(bit(7) | bit(6));

        // Hypervisor pages are inner-shareable, so as to be
        // coherent across PEs, (i.e, SH == 11)
        descriptor |= bit(9) | bit(8);

        /*
         * In ARMv8, software must manage the access flag.
         * If it is NOT set to 1, then attempts at loadding this
         * entry into the TLB will cause an Access flag fault.
         */
        descriptor |= bit(10);

        /* Set to non-Global = 0, because we don't support ASIDs yet.
         * TODO: Support ASIDs
         */
        descriptor &= !bit(11);

        /* TODO: implement Upper attributes */

        return PageTableEntry(descriptor);
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
