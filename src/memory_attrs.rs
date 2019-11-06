/**
 * This module supports the configuration of ARMv8 memory attributes.
 *
 * This primarily deals with the shareability and cacheability
 * of hypervisor memory.
 *
 * Refer to MAIR0/MAIR1 documentation in the ARM Reference Manual
 * or the ARM Cortex-A Programmer's Guide.
 */ 

use crate::msr;

/**
 * For right now we only configure "Normal, Write-back, Write-allocate"
 * memory.  This is what will be used by the hypervisor.  Once we
 * support I/O, obviously Device memory will need to be supported.
 * Once we support guests and pinning, other shareability/cacheability
 * attributes will need support.
 */
pub fn init() -> () {
    /*
     * For the sake of simplicity, let's just use use AttrIndex 0 for
     * Normal Write-Back Write-Allocate memory:
     *
     * Attr @ Index 0 of MAIR0 is Normal Write-bAck Write-Allocate memory
     */

    /* Write to MAIR_EL2 */
    //msr!("mair_el2", 0xff);

    msr!("mair_el2", 0xff);

    /*
    unsafe {
        asm!("msr mair_el2, $0" :: "r"(MAIR) :: );
    }*/
    /*
        ldr   x0, =MAIRVAL
        msr   mair_el2, x0
    */
}
