#![allow(dead_code)]

use crate::uart::uart_write;
use crate::common::bit;
use crate::common::print_hex;
use crate::{msr, mrs};


pub fn get_phys_addr_range() -> u64 {
    let mmfr0: u64;

    mrs!(mmfr0, "ID_AA64MMFR0_EL1"); 

    // ID_AA64MMFR0_EL1[3:0] is PARange
    mmfr0 & 0xf
}


fn alloc_stage2_entry() -> () {
    // Set AF bit
    // Set Read bit
    // Set Table bit
    // Set Valid bit
    // Write address with << PAGE_SHIFT
}


/* Example data
 * TTBR0_EL1      0x6b6a6000          1802133504
 * VTTBR_EL2      0x1000044006000     281476117585920
 * HCR_EL2        0x8078663f          2155374143
 */

fn map_guest_page() -> () {
    // Get the page table root
    // Traverse tree to entry for guest address
    // Allocate an entry for stage2
    alloc_stage2_entry();
}

// VTCR_EL2.TOSZ == VTCR_EL2[5:0]
const VTCR_EL2_T0SZ_MASK: u64 = 0b11111;

// VTCR_EL2.SL0 == VTCR_EL2[7:6]
const VTCR_EL2_SL0_SHIFT: u64 =  5;
const VTCR_EL2_SL0_MASK: u64 =  0b11 << VTCR_EL2_SL0_SHIFT;

#[allow(non_camel_case_types)]
struct VTCR_EL2 {}

impl VTCR_EL2 {
    #[inline]
    pub fn get() -> u64 {
        let reg: u64;

        mrs!(reg, "VTCR_EL2");

        reg
    }

    #[inline]
    pub fn set(val: u64) -> () {
        msr!("VTCR_EL2", val);
    }

    #[inline]
    pub fn t0sz() -> u64 {
        const MASK: u64 = 0b11111;

        return VTCR_EL2::get() & MASK;
    }

    #[inline]
    pub fn set_t0sz(val: u64) {
        const SHIFT: u64 = 0;
        const MASK: u64 =  0b11111 << SHIFT;

        let mut reg = VTCR_EL2::get();

        reg &= !MASK;
        reg |= (val << SHIFT) & MASK;

        msr!("VTCR_EL2", reg);
    }

    #[inline]
    pub fn set_sh0(val: u64) -> () {
        const SHIFT: u64 = 12;
        const MASK: u64 =  0b11 << SHIFT;

        let mut reg = VTCR_EL2::get();

        reg &= !MASK;
        reg |= (val << SHIFT) & MASK;

        msr!("VTCR_EL2", reg);
    }

    #[inline]
    pub fn set_sl0(val: u64) -> () {
        const SHIFT: u64 = 6;
        const MASK: u64 =  0b11 << SHIFT;

        let mut reg = VTCR_EL2::get();

        reg &= !MASK;
        reg |= (val << SHIFT) & MASK;

        msr!("VTCR_EL2", reg);
    }


    #[inline]
    pub fn set_tg0(val: u64) -> () {
        const SHIFT: u64 = 14;
        const MASK: u64 =  0b11 << SHIFT;

        let mut reg = VTCR_EL2::get();

        reg &= !MASK;
        reg |= (val << SHIFT) & MASK;

        msr!("VTCR_EL2", reg);
    }

    #[inline]
    pub fn set_ps(val: u64) -> () {
        const SHIFT: u64 = 16;

        let mut reg = VTCR_EL2::get();

        reg |= val << SHIFT;

        msr!("VTCR_EL2", reg);
    }

    #[inline]
    pub fn reserved() -> () {
        // RES0 [63:32], [24:23], 20
        const RES1: u64 = 1 << 31;
        const RES0_CLR: u64 = (1<<(63-32)) - 1;
        const RES0: u64 = RES0_CLR << 32 | bit(24) | bit(23) | bit(20);

        let mut reg = VTCR_EL2::get();
        reg |= RES1;
        msr!("VTCR_EL2", reg);

        assert!((reg & RES0) == 0);
    }
}


pub fn show_vtcr_el2() -> () {
    uart_write("VTCR_EL2: ");
    print_hex(VTCR_EL2::get());
    uart_write("\n");
    uart_write("VTCR_EL2.T0SZ: ");

    let val = VTCR_EL2::t0sz();
    print_hex(val);
    uart_write("\n");
}

const VTCR_EL2_T0SZ: u64 = 24; // 64 bit width - 40 bit address size
const VTCR_EL2_SH0: u64 = 0x3 << 12; // Inner Shareable
const VTCR_EL2_RES1: u64 = 1 << 31;


pub fn init_vtcr() -> () {
    /* A53
     *
    let pa_range: u64 = 2;
    let pa_bits: u64 = 40;
    let t0sz: u64 = 24;
    let root_order: u64 = 1;
    let sl0: u64 = 1;
    const TG0_4K: u64 = 0;

    VTCR_EL2::set_sh0(0x3);
    VTCR_EL2::set_t0sz(t0sz);
    VTCR_EL2::set_tg0(TG0_4K);
    VTCR_EL2::set_sl0(sl0);
    VTCR_EL2::set_ps(pa_range);
    VTCR_EL2::reserved();
    */

    /*
     * PS    = 010    (2) # 40-bit Phys Addr
     * TG0   = 00     (0) # 4KB Translation Granule
     * SH0   = 11     (3)
     * ORGN0 = 01     (1)
     * IRGN0 = 01     (1)
     * SL0   = 01     (start at level 1)
     * T0SZ  = 011000 (24)
     *
     * RES0=0000_0000_0000_0000_0000_0000_0000_0000 RES1=1 NSa=0 NSw=0 HWx=0_000 RES=00 HD=0 HA=0   RES=0 VS=0_010_00_11_01_01_01_011000
     */

    /* A53 */
    VTCR_EL2::set(0x80023558);

    /* A57 */
    // RES1=1, RES0=000000000000,  PS=100 (44-bit Phys A), TG0=00 (4KB Granule), SH0=11, ORGN0=01, IRGN0=01, SL0=10 (start at level 0), TOSZ=010100 (20)
    //VTCR_EL2::set(0x80043594);
}

