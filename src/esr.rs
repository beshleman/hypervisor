#![allow(non_upper_case_globals)]

/*
 * Based on Linux's arm64/kernel/esr.h
 */

use crate::mrs;
use crate::uart::uart_write;

const ESR_ELx_EC_UNKNOWN: u64 = (0x00);
const ESR_ELx_EC_WFx: u64 =  (0x01);
/* Unallocated EC: 0x02 */
const ESR_ELx_EC_CP15_32: u64 = (0x03);
const ESR_ELx_EC_CP15_64: u64 = (0x04);
const ESR_ELx_EC_CP14_MR: u64 = (0x05);
const ESR_ELx_EC_CP14_LS : u64 =(0x06);
const ESR_ELx_EC_FP_ASIMD: u64 = (0x07);
const ESR_ELx_EC_CP10_ID: u64 = (0x08); /* EL2 only */
const ESR_ELx_EC_PAC: u64 =  (0x09); /* EL2 and above */
/* Unallocated EC: 0x0A - 0x0B */
const ESR_ELx_EC_CP14_64: u64 = (0x0C);
/* Unallocated EC: 0x0d */
const ESR_ELx_EC_ILL: u64 =  (0x0E);
/* Unallocated EC: 0x0F - 0x10 */
const ESR_ELx_EC_SVC32: u64 = (0x11);
const ESR_ELx_EC_HVC32: u64 = (0x12); /* EL2 only */
const ESR_ELx_EC_SMC32: u64 = (0x13); /* EL2 and above */
/* Unallocated EC: 0x14 */
const ESR_ELx_EC_SVC64: u64 = (0x15);
const ESR_ELx_EC_HVC64: u64 = (0x16); /* EL2 and above */
const ESR_ELx_EC_SMC64: u64 = (0x17); /* EL2 and above */
const ESR_ELx_EC_SYS64: u64 = (0x18);
const ESR_ELx_EC_SVE: u64 =  (0x19);
/* Unallocated EC: 0x1A - 0x1E */
const ESR_ELx_EC_IMP_DEF: u64 = (0x1f); /* EL3 only */
const ESR_ELx_EC_IABT_LOW: u64 = (0x20);
const ESR_ELx_EC_IABT_CUR: u64 = (0x21);
const ESR_ELx_EC_PC_ALIGN: u64 = (0x22);
/* Unallocated EC: 0x23 */
const ESR_ELx_EC_DABT_LOW: u64 = (0x24);
const ESR_ELx_EC_DABT_CUR: u64 = (0x25);
const ESR_ELx_EC_SP_ALIGN: u64 = (0x26);
/* Unallocated EC: 0x27 */
const ESR_ELx_EC_FP_EXC32: u64 = (0x28);
/* Unallocated EC: 0x29 - 0x2B */
const ESR_ELx_EC_FP_EXC64: u64 = (0x2C);
/* Unallocated EC: 0x2D - 0x2E */
const ESR_ELx_EC_SERROR: u64 = (0x2F);
const ESR_ELx_EC_BREAKPT_LOW: u64 = (0x30);
const ESR_ELx_EC_BREAKPT_CUR: u64 = (0x31);
const ESR_ELx_EC_SOFTSTP_LOW: u64 = (0x32);
const ESR_ELx_EC_SOFTSTP_CUR: u64 = (0x33);
const ESR_ELx_EC_WATCHPT_LOW: u64 = (0x34);
const ESR_ELx_EC_WATCHPT_CUR: u64 = (0x35);
/* Unallocated EC: 0x36 - 0x37 */
const ESR_ELx_EC_BKPT32: u64 = (0x38);
/* Unallocated EC: 0x39 */
const ESR_ELx_EC_VECTOR32: u64 = (0x3A); /* EL2 only */
/* Unallocted EC: 0x3B */
const ESR_ELx_EC_BRK64: u64 = (0x3C);
/* Unallocated EC: 0x3D - 0x3F */
const ESR_ELx_EC_MAX: u64 =  (0x3F);

const ESR_ELx_EC_SHIFT: u64 = (26);
const ESR_ELx_EC_MASK: u64 =  ((0x3F) << ESR_ELx_EC_SHIFT);

pub const fn esr_elx_ec(esr: u64) -> u64 {
    (esr & ESR_ELx_EC_MASK) >> ESR_ELx_EC_SHIFT
}

const ESR_ELx_IL_SHIFT: u64 = (25);
const ESR_ELx_IL: u64 =  ((1) << ESR_ELx_IL_SHIFT);
const ESR_ELx_ISS_MASK: u64 = (ESR_ELx_IL - 1);

/*
/* ISS field definitions shared by different classes */
const ESR_ELx_WNR_SHIFT: u64 = (6)
const ESR_ELx_WNR: u64 =  ((1) << ESR_ELx_WNR_SHIFT)

/* Asynchronous Error Type */
const ESR_ELx_IDS_SHIFT: u64 = (24)
const ESR_ELx_IDS: u64 =  ((1) << ESR_ELx_IDS_SHIFT)
const ESR_ELx_AET_SHIFT: u64 = (10)
const ESR_ELx_AET: u64 =  ((0x7) << ESR_ELx_AET_SHIFT)

const ESR_ELx_AET_UC: u64 =  ((0) << ESR_ELx_AET_SHIFT)
const ESR_ELx_AET_UEU: u64 =  ((1) << ESR_ELx_AET_SHIFT)
const ESR_ELx_AET_UEO: u64 =  ((2) << ESR_ELx_AET_SHIFT)
const ESR_ELx_AET_UER: u64 =  ((3) << ESR_ELx_AET_SHIFT)
const ESR_ELx_AET_CE: u64 =  ((6) << ESR_ELx_AET_SHIFT)

/* Shared ISS field definitions for Data/Instruction aborts */
const ESR_ELx_SET_SHIFT: u64 = (11)
const ESR_ELx_SET_MASK: u64 = ((3) << ESR_ELx_SET_SHIFT)
const ESR_ELx_FnV_SHIFT: u64 = (10)
const ESR_ELx_FnV: u64 =  ((1) << ESR_ELx_FnV_SHIFT)
const ESR_ELx_EA_SHIFT: u64 = (9)
const ESR_ELx_EA: u64 =  ((1) << ESR_ELx_EA_SHIFT)
const ESR_ELx_S1PTW_SHIFT: u64 = (7)
const ESR_ELx_S1PTW: u64 =  ((1) << ESR_ELx_S1PTW_SHIFT)

/* Shared ISS fault status code(IFSC/DFSC) for Data/Instruction aborts */
const ESR_ELx_FSC: u64 =  (0x3F)
const ESR_ELx_FSC_TYPE: u64 = (0x3C)
const ESR_ELx_FSC_EXTABT: u64 = (0x10)
const ESR_ELx_FSC_SERROR: u64 = (0x11)
const ESR_ELx_FSC_ACCESS: u64 = (0x08)
const ESR_ELx_FSC_FAULT: u64 = (0x04)
const ESR_ELx_FSC_PERM: u64 = (0x0C)

/* ISS field definitions for Data Aborts */
const ESR_ELx_ISV_SHIFT: u64 = (24)
const ESR_ELx_ISV: u64 =  ((1) << ESR_ELx_ISV_SHIFT)
const ESR_ELx_SAS_SHIFT: u64 = (22)
const ESR_ELx_SAS: u64 =  ((3) << ESR_ELx_SAS_SHIFT)
const ESR_ELx_SSE_SHIFT: u64 = (21)
const ESR_ELx_SSE: u64 =  ((1) << ESR_ELx_SSE_SHIFT)
const ESR_ELx_SRT_SHIFT: u64 = (16)
const ESR_ELx_SRT_MASK: u64 = ((0x1F) << ESR_ELx_SRT_SHIFT)
const ESR_ELx_SF_SHIFT: u64 = (15)
const ESR_ELx_SF: u64 =   ((1) << ESR_ELx_SF_SHIFT)
const ESR_ELx_AR_SHIFT: u64 = (14)
const ESR_ELx_AR: u64 =   ((1) << ESR_ELx_AR_SHIFT)
const ESR_ELx_CM_SHIFT: u64 = (8)
const ESR_ELx_CM: u64 =   ((1) << ESR_ELx_CM_SHIFT)

/* ISS field definitions for exceptions taken in to Hyp */
const ESR_ELx_CV: u64 =  ((1) << 24)
const ESR_ELx_COND_SHIFT: u64 = (20)
const ESR_ELx_COND_MASK: u64 = ((0xF) << ESR_ELx_COND_SHIFT)
const ESR_ELx_WFx_ISS_TI: u64 = ((1) << 0)
const ESR_ELx_WFx_ISS_WFI: u64 = ((0) << 0)
const ESR_ELx_WFx_ISS_WFE: u64 = ((1) << 0)
const ESR_ELx_xVC_IMM_MASK: u64 = ((1 << 16) - 1)

const DISR_EL1_IDS: u64 =  ((1) << 24)
/*
 * DISR_EL1 and ESR_ELx share the bottom 13 bits, but the RES0 bits may mean
 * different things in the future...
 */
const DISR_EL1_ESR_MASK: u64 = (ESR_ELx_AET | ESR_ELx_EA | ESR_ELx_FSC)

/* ESR value templates for specific events */
const ESR_ELx_WFx_MASK: u64 = (ESR_ELx_EC_MASK | ESR_ELx_WFx_ISS_TI)
const ESR_ELx_WFx_WFI_VAL: u64 = ((ESR_ELx_EC_WFx << ESR_ELx_EC_SHIFT) | \
     ESR_ELx_WFx_ISS_WFI)

/* BRK instruction trap from AArch64 state */
const ESR_ELx_BRK64_ISS_COMMENT_MASK: u64 = 0xffff

/* ISS field definitions for System instruction traps */
const ESR_ELx_SYS64_ISS_RES0_SHIFT: u64 = 22
const ESR_ELx_SYS64_ISS_RES0_MASK: u64 = ((0x7) << ESR_ELx_SYS64_ISS_RES0_SHIFT)
const ESR_ELx_SYS64_ISS_DIR_MASK: u64 = 0x1
const ESR_ELx_SYS64_ISS_DIR_READ: u64 = 0x1
const ESR_ELx_SYS64_ISS_DIR_WRITE: u64 = 0x0

const ESR_ELx_SYS64_ISS_RT_SHIFT: u64 = 5
const ESR_ELx_SYS64_ISS_RT_MASK: u64 = ((0x1f) << ESR_ELx_SYS64_ISS_RT_SHIFT)
const ESR_ELx_SYS64_ISS_CRM_SHIFT: u64 = 1
const ESR_ELx_SYS64_ISS_CRM_MASK: u64 = ((0xf) << ESR_ELx_SYS64_ISS_CRM_SHIFT)
const ESR_ELx_SYS64_ISS_CRN_SHIFT: u64 = 10
const ESR_ELx_SYS64_ISS_CRN_MASK: u64 = ((0xf) << ESR_ELx_SYS64_ISS_CRN_SHIFT)
const ESR_ELx_SYS64_ISS_OP1_SHIFT: u64 = 14
const ESR_ELx_SYS64_ISS_OP1_MASK: u64 = ((0x7) << ESR_ELx_SYS64_ISS_OP1_SHIFT)
const ESR_ELx_SYS64_ISS_OP2_SHIFT: u64 = 17
const ESR_ELx_SYS64_ISS_OP2_MASK: u64 = ((0x7) << ESR_ELx_SYS64_ISS_OP2_SHIFT)
const ESR_ELx_SYS64_ISS_OP0_SHIFT: u64 = 20
const ESR_ELx_SYS64_ISS_OP0_MASK: u64 = ((0x3) << ESR_ELx_SYS64_ISS_OP0_SHIFT)
const ESR_ELx_SYS64_ISS_SYS_MASK: u64 = (ESR_ELx_SYS64_ISS_OP0_MASK | \
      ESR_ELx_SYS64_ISS_OP1_MASK | \
      ESR_ELx_SYS64_ISS_OP2_MASK | \
      ESR_ELx_SYS64_ISS_CRN_MASK | \
      ESR_ELx_SYS64_ISS_CRM_MASK)
const ESR_ELx_SYS64_ISS_SYS_VAL(op0, op1, op2, crn, crm) \
     (((op0) << ESR_ELx_SYS64_ISS_OP0_SHIFT) | \
      ((op1) << ESR_ELx_SYS64_ISS_OP1_SHIFT) | \
      ((op2) << ESR_ELx_SYS64_ISS_OP2_SHIFT) | \
      ((crn) << ESR_ELx_SYS64_ISS_CRN_SHIFT) | \
      ((crm) << ESR_ELx_SYS64_ISS_CRM_SHIFT))

const ESR_ELx_SYS64_ISS_SYS_OP_MASK: u64 = (ESR_ELx_SYS64_ISS_SYS_MASK | \
      ESR_ELx_SYS64_ISS_DIR_MASK)
const ESR_ELx_SYS64_ISS_RT(esr): u64 = \
 (((esr) & ESR_ELx_SYS64_ISS_RT_MASK) >> ESR_ELx_SYS64_ISS_RT_SHIFT)
/*
 * User space cache operations have the following sysreg encoding
 * in System instructions.
 * op0=1, op1=3, op2=1, crn=7, crm={ 5, 10, 11, 12, 13, 14 }, WRITE (L=0)
 */
const ESR_ELx_SYS64_ISS_CRM_DC_CIVAC: u64 = 14
const ESR_ELx_SYS64_ISS_CRM_DC_CVADP: u64 = 13
const ESR_ELx_SYS64_ISS_CRM_DC_CVAP: u64 = 12
const ESR_ELx_SYS64_ISS_CRM_DC_CVAU: u64 = 11
const ESR_ELx_SYS64_ISS_CRM_DC_CVAC: u64 = 10
const ESR_ELx_SYS64_ISS_CRM_IC_IVAU: u64 = 5

const ESR_ELx_SYS64_ISS_EL0_CACHE_OP_MASK: u64 = (ESR_ELx_SYS64_ISS_OP0_MASK | \
       ESR_ELx_SYS64_ISS_OP1_MASK | \
       ESR_ELx_SYS64_ISS_OP2_MASK | \
       ESR_ELx_SYS64_ISS_CRN_MASK | \
       ESR_ELx_SYS64_ISS_DIR_MASK)
const ESR_ELx_SYS64_ISS_EL0_CACHE_OP_VAL: u64 = \
    (ESR_ELx_SYS64_ISS_SYS_VAL(1, 3, 1, 7, 0) | \
     ESR_ELx_SYS64_ISS_DIR_WRITE)
/*
 * User space MRS operations which are supported for emulation
 * have the following sysreg encoding in System instructions.
 * op0 = 3, op1= 0, crn = 0, {crm = 0, 4-7}, READ (L = 1)
 */
const ESR_ELx_SYS64_ISS_SYS_MRS_OP_MASK: u64 = (ESR_ELx_SYS64_ISS_OP0_MASK | \
       ESR_ELx_SYS64_ISS_OP1_MASK | \
       ESR_ELx_SYS64_ISS_CRN_MASK | \
       ESR_ELx_SYS64_ISS_DIR_MASK)
const ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL: u64 = \
    (ESR_ELx_SYS64_ISS_SYS_VAL(3, 0, 0, 0, 0) | \
     ESR_ELx_SYS64_ISS_DIR_READ)

const ESR_ELx_SYS64_ISS_SYS_CTR: u64 = ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 1, 0, 0)
const ESR_ELx_SYS64_ISS_SYS_CTR_READ: u64 = (ESR_ELx_SYS64_ISS_SYS_CTR | \
      ESR_ELx_SYS64_ISS_DIR_READ)

const ESR_ELx_SYS64_ISS_SYS_CNTVCT: u64 = (ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 2, 14, 0) | \
      ESR_ELx_SYS64_ISS_DIR_READ)

const ESR_ELx_SYS64_ISS_SYS_CNTFRQ: u64 = (ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 0, 14, 0) | \
      ESR_ELx_SYS64_ISS_DIR_READ)

const esr_sys64_to_sysreg(e)     \
 sys_reg((((e) & ESR_ELx_SYS64_ISS_OP0_MASK) >>  \
   ESR_ELx_SYS64_ISS_OP0_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_OP1_MASK) >>  \
   ESR_ELx_SYS64_ISS_OP1_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_CRN_MASK) >>  \
   ESR_ELx_SYS64_ISS_CRN_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_CRM_MASK) >>  \
   ESR_ELx_SYS64_ISS_CRM_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_OP2_MASK) >>  \
   ESR_ELx_SYS64_ISS_OP2_SHIFT))

const esr_cp15_to_sysreg(e)     \
 sys_reg(3,      \
  (((e) & ESR_ELx_SYS64_ISS_OP1_MASK) >>  \
   ESR_ELx_SYS64_ISS_OP1_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_CRN_MASK) >>  \
   ESR_ELx_SYS64_ISS_CRN_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_CRM_MASK) >>  \
   ESR_ELx_SYS64_ISS_CRM_SHIFT),   \
  (((e) & ESR_ELx_SYS64_ISS_OP2_MASK) >>  \
   ESR_ELx_SYS64_ISS_OP2_SHIFT))

/*
 * ISS field definitions for floating-point exception traps
 * (FP_EXC_32/FP_EXC_64).
 *
 * (The FPEXC_* constants are used instead for common bits.)
 */

const ESR_ELx_FP_EXC_TFV: u64 = ((1) << 23)

/*
 * ISS field definitions for CP15 accesses
 */
const ESR_ELx_CP15_32_ISS_DIR_MASK: u64 = 0x1
const ESR_ELx_CP15_32_ISS_DIR_READ: u64 = 0x1
const ESR_ELx_CP15_32_ISS_DIR_WRITE: u64 = 0x0

const ESR_ELx_CP15_32_ISS_RT_SHIFT: u64 = 5
const ESR_ELx_CP15_32_ISS_RT_MASK: u64 = ((0x1f) << ESR_ELx_CP15_32_ISS_RT_SHIFT)
const ESR_ELx_CP15_32_ISS_CRM_SHIFT: u64 = 1
const ESR_ELx_CP15_32_ISS_CRM_MASK: u64 = ((0xf) << ESR_ELx_CP15_32_ISS_CRM_SHIFT)
const ESR_ELx_CP15_32_ISS_CRN_SHIFT: u64 = 10
const ESR_ELx_CP15_32_ISS_CRN_MASK: u64 = ((0xf) << ESR_ELx_CP15_32_ISS_CRN_SHIFT)
const ESR_ELx_CP15_32_ISS_OP1_SHIFT: u64 = 14
const ESR_ELx_CP15_32_ISS_OP1_MASK: u64 = ((0x7) << ESR_ELx_CP15_32_ISS_OP1_SHIFT)
const ESR_ELx_CP15_32_ISS_OP2_SHIFT: u64 = 17
const ESR_ELx_CP15_32_ISS_OP2_MASK: u64 = ((0x7) << ESR_ELx_CP15_32_ISS_OP2_SHIFT)

const ESR_ELx_CP15_32_ISS_SYS_MASK: u64 = (ESR_ELx_CP15_32_ISS_OP1_MASK | \
      ESR_ELx_CP15_32_ISS_OP2_MASK | \
      ESR_ELx_CP15_32_ISS_CRN_MASK | \
      ESR_ELx_CP15_32_ISS_CRM_MASK | \
      ESR_ELx_CP15_32_ISS_DIR_MASK)
/*
const ESR_ELx_CP15_32_ISS_SYS_VAL(op1, op2, crn, crm) \
     (((op1) << ESR_ELx_CP15_32_ISS_OP1_SHIFT) | \
      ((op2) << ESR_ELx_CP15_32_ISS_OP2_SHIFT) | \
      ((crn) << ESR_ELx_CP15_32_ISS_CRN_SHIFT) | \
      ((crm) << ESR_ELx_CP15_32_ISS_CRM_SHIFT))
*/

const ESR_ELx_CP15_64_ISS_DIR_MASK: u64 = 0x1
const ESR_ELx_CP15_64_ISS_DIR_READ: u64 = 0x1
const ESR_ELx_CP15_64_ISS_DIR_WRITE: u64 = 0x0

const ESR_ELx_CP15_64_ISS_RT_SHIFT: u64 = 5
const ESR_ELx_CP15_64_ISS_RT_MASK: u64 = ((0x1f) << ESR_ELx_CP15_64_ISS_RT_SHIFT)

const ESR_ELx_CP15_64_ISS_RT2_SHIFT: u64 = 10
const ESR_ELx_CP15_64_ISS_RT2_MASK: u64 = ((0x1f) << ESR_ELx_CP15_64_ISS_RT2_SHIFT)

const ESR_ELx_CP15_64_ISS_OP1_SHIFT: u64 = 16
const ESR_ELx_CP15_64_ISS_OP1_MASK: u64 = ((0xf) << ESR_ELx_CP15_64_ISS_OP1_SHIFT)
const ESR_ELx_CP15_64_ISS_CRM_SHIFT: u64 = 1
const ESR_ELx_CP15_64_ISS_CRM_MASK: u64 = ((0xf) << ESR_ELx_CP15_64_ISS_CRM_SHIFT)

const ESR_ELx_CP15_64_ISS_SYS_MASK: u64 = (ESR_ELx_CP15_64_ISS_OP1_MASK | \
      ESR_ELx_CP15_64_ISS_CRM_MASK | \
      ESR_ELx_CP15_64_ISS_DIR_MASK)

const ESR_ELx_CP15_64_ISS_SYS_CNTVCT: u64 = (ESR_ELx_CP15_64_ISS_SYS_VAL(1, 14) | \
      ESR_ELx_CP15_64_ISS_DIR_READ)

const ESR_ELx_CP15_32_ISS_SYS_CNTFRQ: u64 = (ESR_ELx_CP15_32_ISS_SYS_VAL(0, 0, 14, 0) |\
      ESR_ELx_CP15_32_ISS_DIR_READ)
*/

pub enum ExceptionLevel {
    EL2
}

pub fn esr(el: ExceptionLevel) -> u64 {
    let esr_el2;

    match el {
        ExceptionLevel::EL2 => {
            mrs!(esr_el2, "ESR_EL2");
        }
    }

    esr_el2
}

const IFSC_MASK: u64 = ((1 << 6) - 1);

pub fn print_inst_abort_current(esr_el2: u64) -> () {
    uart_write("Instruction Abort Current ELx\n");


    let ifsc = esr_el2 & IFSC_MASK;

    let mut _myifsc = ifsc;

    let string = 
        match ifsc {
            0b000000 => "Address Size Fault, level 0 or TTBR",
            0b000001 => "Address Size Fault, level 1",
            0b000010 => "Address Size Fault, level 2",
            0b000011 => "Address Size Fault, level 3",
            0b000100 => "Translation fault, level 0",
            0b000101 => "Translation fault, level 1",
            0b000110 => "Translation fault, level 2",
            0b000111 => "Translation fault, level 3",
            0b001001 => "Access flag fault, level 1",
            0b001010 => "Access flag fault, level 2",
            0b001011 => "Access flag fault, level 3",
            0b001101 => "Permission fault, level 1",
            0b001110 => "Permission fault, level 2",
            0b001111 => "Permission fault, level 3",
            0b010000 => "Synchronous External abort, not on translation table walk",
            0b010100 => "Synchronous External abort, on translation table walk, level 0",
            0b010101 => "Synchronous External abort, on translation table walk, level 1",
            0b010110 => "Synchronous External abort, on translation table walk, level 2",
            0b010111 => "Synchronous External abort, on translation table walk, level 3",
            0b011000 => "Synchronous parity or ECC error on memory access, not on translation table walk",
            0b011100 => "Synchronous parity or ECC error on memory access on translation table walk, level 0",
            0b011101 => "Synchronous parity or ECC error on memory access on translation table walk, level 1",
            0b011110 => "Synchronous parity or ECC error on memory access on translation table walk, level 2",
            0b011111 => "Synchronous parity or ECC error on memory access on translation table walk, level 3",
            0b110000 => "TLB conflict abort",
            _ => "Other",
        };

    uart_write("Instruction Fault Status Code: ");
    uart_write(string);
    uart_write("\n");
}

pub fn print_exception_syndrome() -> () {
    let esr_el2 = esr(ExceptionLevel::EL2);
    let ec = esr_elx_ec(esr_el2);

    match ec {
        ESR_ELx_EC_WFx => uart_write("ESR_ELx_EC_WFx"),
        ESR_ELx_EC_CP15_32 => uart_write("ESR_ELx_EC_CP15_32"),
        ESR_ELx_EC_CP15_64 => uart_write("ESR_ELx_EC_CP15_64"),
        ESR_ELx_EC_CP14_MR => uart_write("ESR_ELx_EC_CP14_MR"),
        ESR_ELx_EC_CP14_LS => uart_write("ESR_ELx_EC_CP14_LS"),
        ESR_ELx_EC_FP_ASIMD => uart_write("ESR_ELx_EC_FP_ASIMD"),
        ESR_ELx_EC_CP10_ID => uart_write("ESR_ELx_EC_CP10_ID"),
        ESR_ELx_EC_PAC => uart_write("ESR_ELx_EC_PAC"),
        ESR_ELx_EC_CP14_64 => uart_write("ESR_ELx_EC_CP14_64"),
        ESR_ELx_EC_ILL => uart_write("ESR_ELx_EC_ILL"),
        ESR_ELx_EC_SVC32 => uart_write("ESR_ELx_EC_SVC32"),
        ESR_ELx_EC_HVC32 => uart_write("ESR_ELx_EC_HVC32"),
        ESR_ELx_EC_SMC32 => uart_write("ESR_ELx_EC_SMC32"),
        ESR_ELx_EC_SVC64 => uart_write("ESR_ELx_EC_SVC64"),
        ESR_ELx_EC_HVC64 => uart_write("ESR_ELx_EC_HVC64"),
        ESR_ELx_EC_SMC64 => uart_write("ESR_ELx_EC_SMC64"),
        ESR_ELx_EC_SYS64 => uart_write("ESR_ELx_EC_SYS64"),
        ESR_ELx_EC_SVE => uart_write("ESR_ELx_EC_SVE"),
        ESR_ELx_EC_IMP_DEF => uart_write("ESR_ELx_EC_IMP_DEF"),
        ESR_ELx_EC_IABT_LOW => uart_write("Instruction Abort Lower ELx"),
        ESR_ELx_EC_IABT_CUR => print_inst_abort_current(esr_el2),
        ESR_ELx_EC_PC_ALIGN => uart_write("ESR_ELx_EC_PC_ALIGN"),
        ESR_ELx_EC_DABT_LOW => uart_write("ESR_ELx_EC_DABT_LOW"),
        ESR_ELx_EC_DABT_CUR => uart_write("ESR_ELx_EC_DABT_CUR"),
        ESR_ELx_EC_SP_ALIGN => uart_write("ESR_ELx_EC_SP_ALIGN"),
        ESR_ELx_EC_FP_EXC32 => uart_write("ESR_ELx_EC_FP_EXC32"),
        ESR_ELx_EC_FP_EXC64 => uart_write("ESR_ELx_EC_FP_EXC64"),
        ESR_ELx_EC_SERROR => uart_write("ESR_ELx_EC_SERROR"),
        ESR_ELx_EC_BREAKPT_LOW => uart_write("ESR_ELx_EC_BREAKPT_LOW"),
        ESR_ELx_EC_BREAKPT_CUR => uart_write("ESR_ELx_EC_BREAKPT_CUR"),
        ESR_ELx_EC_SOFTSTP_LOW => uart_write("ESR_ELx_EC_SOFTSTP_LOW"),
        ESR_ELx_EC_SOFTSTP_CUR => uart_write("ESR_ELx_EC_SOFTSTP_CUR"),
        ESR_ELx_EC_WATCHPT_LOW => uart_write("ESR_ELx_EC_WATCHPT_LOW"),
        ESR_ELx_EC_WATCHPT_CUR => uart_write("ESR_ELx_EC_WATCHPT_CUR"),
        ESR_ELx_EC_BKPT32 => uart_write("ESR_ELx_EC_BKPT32"),
        ESR_ELx_EC_VECTOR32 => uart_write("ESR_ELx_EC_VECTOR32"),
        ESR_ELx_EC_BRK64 => uart_write("ESR_ELx_EC_BRK64"),
        ESR_ELx_EC_MAX => uart_write("ESR_ELx_EC_MAX"),
        /*
        ESR_ELx_SYS64_ISS_DIR_READ => uart_write("ESR_ELx_SYS64_ISS_DIR_READ"),
        ESR_ELx_SYS64_ISS_DIR_WRITE => uart_write("ESR_ELx_SYS64_ISS_DIR_WRITE"),
        ESR_ELx_SYS64_ISS_CRM_DC_CIVAC => uart_write("ESR_ELx_SYS64_ISS_CRM_DC_CIVAC"),
        ESR_ELx_SYS64_ISS_CRM_DC_CVADP => uart_write("ESR_ELx_SYS64_ISS_CRM_DC_CVADP"),
        ESR_ELx_SYS64_ISS_CRM_DC_CVAP => uart_write("ESR_ELx_SYS64_ISS_CRM_DC_CVAP"),
        ESR_ELx_SYS64_ISS_CRM_DC_CVAU => uart_write("ESR_ELx_SYS64_ISS_CRM_DC_CVAU"),
        ESR_ELx_SYS64_ISS_CRM_DC_CVAC => uart_write("ESR_ELx_SYS64_ISS_CRM_DC_CVAC"),
        ESR_ELx_SYS64_ISS_CRM_IC_IVAU => uart_write("ESR_ELx_SYS64_ISS_CRM_IC_IVAU"),
        ESR_ELx_SYS64_ISS_EL0_CACHE_OP_VAL => uart_write("ESR_ELx_SYS64_ISS_EL0_CACHE_OP_VAL"),
        ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL => uart_write("ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL"),
        ESR_ELx_SYS64_ISS_SYS_CTR => uart_write("ESR_ELx_SYS64_ISS_SYS_CTR"),
        ESR_ELx_CP15_32_ISS_DIR_READ => uart_write("ESR_ELx_CP15_32_ISS_DIR_READ"),
        ESR_ELx_CP15_32_ISS_DIR_WRITE => uart_write("ESR_ELx_CP15_32_ISS_DIR_WRITE"),
        ESR_ELx_CP15_64_ISS_DIR_READ => uart_write("ESR_ELx_CP15_64_ISS_DIR_READ"),
        ESR_ELx_CP15_64_ISS_DIR_WRITE => uart_write("ESR_ELx_CP15_64_ISS_DIR_WRITE"),
        */
        _ => uart_write("UNKNOWN"),
    }
}
