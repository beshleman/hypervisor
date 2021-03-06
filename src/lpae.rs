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

const THIRD_SHIFT: u64 =  12;
const THIRD_SIZE: u64 =   1 << THIRD_SHIFT;
const THIRD_MASK: u64 =   (!(THIRD_SIZE - 1));

const SECOND_SHIFT: u64 = 21;
const SECOND_SIZE: u64 =  (1 << SECOND_SHIFT);
const SECOND_MASK: u64 =  (!(SECOND_SIZE - 1));

const FIRST_SHIFT: u64 =  30;
const FIRST_SIZE: u64 =   (1 << FIRST_SHIFT);
const FIRST_MASK: u64 =   (!(FIRST_SIZE - 1));

const ZEROETH_SHIFT: u64 = 39;
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

impl PageTableEntry {
    pub unsafe fn as_pagetable(&self) -> &mut PageTable {
        let pt_address: u64 = core::mem::transmute(&self);
        let address = (pt_address & !((1 << 12) - 1)) & CORTEX_A53_MAX_OA;
        return core::mem::transmute(address); 
    }

    pub fn is_valid(&self) -> bool {
        return (self.0 & PTE_VALID) == 1;
    }
}

#[repr(align(8192))]
pub struct PageTableConcat {
    pub entries: [PageTableEntry; 1024],
}

impl PageTableConcat {
    pub fn new() -> PageTableConcat {
        PageTableConcat {
                entries: [PageTableEntry(0); 1024]
        }
    }
}



#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new() -> PageTable {
        PageTable {
                entries: [PageTableEntry(0); 512]
        }
    }
}

//pub type PageTable = [PageTableEntry; 512];

// The AP_TABLE_BITS is RES0 for EL2 w/ no ARMv8.1-VHE
const AP_TABLE_BITS: u64 = bitfield(62, 61);
const TABLE_DESCRIPTOR_RES0: u64 = bitfield(51, 48) | AP_TABLE_BITS;
const STAGE2_TABLE_DESCRIPTOR_RES0: u64 = bitfield(51, 48) | AP_TABLE_BITS | bitfield(63, 59);
const TABLE_NON_SECURE: u64 = bit(63);

// The maxium output address for the Cortex-A53
const CORTEX_A53_MAX_OA: u64 = (1 << 40) - 1;

impl PageTableEntry {
    pub fn from_table(table: &PageTable) -> PageTableEntry {
        let address: u64 = (table as *const PageTable) as u64;
        let mut descriptor: u64 = 0;

        // Set next level table address
        descriptor |=  (address >> 12) << 12;

        // Clear bits beyond 40th bit because Cortex-A53 only
        // supports 40-bit OA
        // descriptor &= !((1 << 40) - 1);
        
        // This is hypervisor memory, so set the Non-Secure Table bit to 1
        descriptor |= TABLE_NON_SECURE;
        descriptor |= PTE_VALID;
        descriptor |= PTE_TABLE;

        assert_eq!(descriptor & TABLE_DESCRIPTOR_RES0, 0);
        return PageTableEntry(descriptor);
    }

    /// Refer to D5.3 for Stage 2 translation table format descriptors
    ///
    /// NOTE: For now, we are using only Normal memory.  This is NOT
    /// good for device memory.  This will need to be changed.
    pub fn from_table_stage2(table: &PageTable) -> PageTableEntry {
        let address: u64 = (table as *const PageTable) as u64;

        // Set next level table address
        let mut descriptor: u64 =  address & !((1<<12)-1);

        // Clear bits beyond 40th bit because Cortex-A53 only
        // supports 40-bit OA
        descriptor &= (1 << 40) - 1;
        
        /*
         * Set valid, table, af, read, sh inner, mem attr to device
         */
        descriptor |= PTE_VALID;
        descriptor |= PTE_TABLE;

        // AF[10] = 1
        descriptor |= 1 << 10;

        // SH[9:8] == Normal, 
        descriptor |= 0b10 << 8;

        // Use memory attr 000, which is inner-shareable, WBWA
        descriptor &= !(bit(4) | bit(3) | bit(2));

        // This is a Non-Secure block, NS == 1
        descriptor |= bit(5);

        // read/write 
        descriptor |= bit(7) | bit(6);

        assert_eq!(descriptor & STAGE2_TABLE_DESCRIPTOR_RES0, 0);
        return PageTableEntry(descriptor);
    }

    pub fn from_block_stage2(address: u64) -> PageTableEntry {
        assert!(address < CORTEX_A53_MAX_OA);

        // Align address to 1GB
        let mut descriptor = address & !((1<<30) - 1);

        // Valid
        descriptor |= 1;

        // Not a table, so bit 1 is 0
        assert_eq!(descriptor & 0b10, 0);

        // S2AP
        descriptor |= bit(7);
        descriptor |= bit(6);

        // Access Flag
        descriptor |= bit(10);
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

        /* For 4K mappings, PTE_TABLE is set too */
        descriptor |= PTE_TABLE;
        descriptor |= PTE_VALID;

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
         * If it is NOT set to 1, then attempts at loading this
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
    ((vaddr >> ZEROETH_SHIFT) & PTE_MASK) as usize
}

pub fn pagetable_concat_table(vaddr: VirtualAddress) -> usize {
    let bit = (vaddr & (1 << 39)) >> 39;

    bit as usize
}

pub fn pagetable_first_index(vaddr: VirtualAddress) -> usize {
    ((vaddr >> FIRST_SHIFT) & PTE_MASK) as usize
}

pub fn pagetable_second_index(vaddr: VirtualAddress) -> usize {
    ((vaddr >> SECOND_SHIFT) & PTE_MASK) as usize
}

pub fn pagetable_third_index(vaddr: VirtualAddress) -> usize {
    return ((vaddr >> THIRD_SHIFT) & PTE_MASK) as usize;
}

pub fn align_4k(addr: u64) -> u64 {
    return addr & ALIGN_4K_MASK;
}

pub struct PageTableTree {
    pub zeroeth: PageTable,
    first: PageTable,
    second: PageTable,
    third: PageTable,
}

impl PageTableTree {
    pub fn new() -> PageTableTree {
        PageTableTree {
            zeroeth: PageTable::new(),
            first: PageTable::new(),
            second: PageTable::new(),
            third: PageTable::new(),
        }
    }

    pub fn map(&mut self, vaddr: u64, paddr: u64) -> () {
        let index0 = pagetable_zeroeth_index(vaddr);
        if self.zeroeth.entries[index0].is_valid() {
            //loop {}
        }

        let index1 = pagetable_first_index(vaddr);
        if self.first.entries[index1].is_valid() {
            //loop {}
        }

        let index2 = pagetable_second_index(vaddr);
        if self.second.entries[index2].is_valid() {
            //loop {}
        }

        let index3 = pagetable_third_index(vaddr);
        if self.third.entries[index3].is_valid() {
            loop {}
        }

        self.zeroeth.entries[index0] = PageTableEntry::from_table(&self.first);
        self.first.entries[index1] = PageTableEntry::from_table(&self.second);
        self.second.entries[index2] = PageTableEntry::from_table(&self.third);
        self.third.entries[index3] = PageTableEntry::from_block(paddr);
    }
}

pub struct PageTableTreeStage2 {
    pub zeroeth: PageTableConcat,
    first: PageTable,
}

impl PageTableTreeStage2 {
    pub fn new() -> PageTableTreeStage2 {
        PageTableTreeStage2 {
            zeroeth: PageTableConcat::new(),
            first: PageTable::new(),
        }
    }

    pub fn map(&mut self, vaddr: u64, paddr: u64) -> () {
        let table_start = pagetable_concat_table(vaddr) * 512;
        let index1 = pagetable_first_index(vaddr);
        let index2 = pagetable_second_index(vaddr);

        self.zeroeth.entries[table_start + index1] = PageTableEntry::from_table_stage2(&self.first);
        self.first.entries[index2] = PageTableEntry::from_block_stage2(paddr);
    }
}

