#include "arm64-asm.h"

	.balign 32
	.global vectors
vectors:
        /* Linux Image header here for U-Boot, taken from tbm */
	/* HW reset vector.  */
	ldr	x16, _start
	br	x16

	/* text offset.  */
	.dword	0
	/* image size.  */
	.dword	0
	/* flags.  */
	.dword	8
	/* RES0  */
	.dword	0
	.dword	0
	.dword	0

	/* magic  */
	.dword	0x644d5241
	/* RES0  */
	.dword	0
	/* End of Image header.  */

	.org	0x200
	b	excp_decode

	.org	0x280
	b	excp_decode

	.org	0x300
	b	excp_decode

	.org	0x380
	b	excp_decode

	.org	0x400
	b	excp_decode

	.org	0x480
	b	excp_decode

	.org	0x500
	b	excp_decode

	.org	0x580
	b	excp_decode

excp_decode:
	store_all_regs
        b _hang

ret_from_excp:
	load_all_regs
	isb
	eret
	.size vectors, . - vectors

	.text

	/* ELF entry point.  */
	.global _start

/* Taken from Xen project
 *
 * Stack pushing/popping (register pairs only). Equivalent to store decrement
 * before, load increment after.
 */
        .macro  push, xreg1, xreg2
        stp     \xreg1, \xreg2, [sp, #-16]!
        .endm

        .macro  pop, xreg1, xreg2
        ldp     \xreg1, \xreg2, [sp], #16
        .endm


/* Save all hypervisor related registers */
        .macro  entry
        sub     sp, sp, #(8 * (15*2))
        push    x28, x29
        push    x26, x27
        push    x24, x25
        push    x22, x23
        push    x20, x21
        push    x18, x19
        push    x16, x17
        push    x14, x15
        push    x12, x13
        push    x10, x11
        push    x8, x9
        push    x6, x7
        push    x4, x5
        push    x2, x3
        push    x0, x1

        mrs     x21, elr_el2
        mrs     x22, spsr_el2
	push	x22, x21

        mrs     x23, esr_el2
        mrs     x23, hcr_el2
	push 	x23, x24
        .endm

/* TODO */
	.macro exit
	.endm

/*
 * Exception vectors.  Taken from Xen project.
 */
        .macro  ventry  label
        .align  7
        b       \label
        .endm

        .align  11

hyp_traps_vector:
        ventry  catchall        /* Synchronous EL2t */
        ventry  catchall        /* IRQ EL2t */
        ventry  catchall        /* FIQ EL2t */
        ventry  catchall        /* Error EL2t */

        ventry  catchall        /* Synchronous EL2h */
        ventry  catchall        /* IRQ EL2h */
        ventry  catchall        /* FIQ EL2h */
        ventry  catchall        /* Error EL2h */

        ventry  catchall        /* Synchronous 64-bit EL0/EL1 */
        ventry  catchall        /* IRQ 64-bit EL0/EL1 */
        ventry  catchall        /* FIQ 64-bit EL0/EL1 */
        ventry  catchall        /* Error 64-bit EL0/EL1 */

        ventry  catchall        /* Synchronous 32-bit EL0/EL1 */
        ventry  catchall        /* IRQ 32-bit EL0/EL1 */
        ventry  catchall    	/* FIQ 32-bit EL0/EL1 */
        ventry  catchall        /* Error 32-bit EL0/EL1 */

/*
 * For now, we just use one handler.
 * TODO: differentiate IRQs
 */
catchall:
        entry
        msr     daifclr, #4
        mov     x0, sp
        bl      irq_handler
        exit

_start:
        /*
         * SMP not yet supported, hang secondary PEs
         * TODO: support SMP
         */ 
	mrs     x2, mpidr_el1
	and	x2, x2, 0xff
	cbnz	x2, _hang
1:
        /* Allow SP_ELx to be used in ELx exceptions */
	mov	x1, #1
	msr	spsel, x1
	isb

        /* x10 = the offset of our physical location */
        adr x11, _start
        ldr x12, =_start
        sub x10, x11, x12
        
        /*
         * Zero BSS
         * x1 = phys_addr(_start_bss)
         * x2 = phys_addr(_end_bss) 
         */
	ldr	x1, =_start_bss
        add     x1, x1, x10
	ldr	x2, =_end_bss
        add     x2, x2, x10

1:
        str     xzr, [x1], #8
        cmp     x1, x2
        b.lo    1b

        /*
         * Zero stack
         * x1 = phys_addr(_start_stack)
         * x2 = phys_addr(_end_stack) 
         */
	ldr	x1, =start_stack
        add     x1, x1, x10
	ldr	x2, =end_stack
        add     x2, x2, x10

1:
        str     xzr, [x1], #8
        cmp     x1, x2
        b.lo    1b

        /* Update the stack pointer */
	ldr	x1, =end_stack
	mov	sp, x1

	/* Pass start, end, and phys offset to Rust */
	/* x0 = start, x1 = end, x2 = offset */
        adr 	x0, _start
        adr 	x1, _end
	mov    x2, x10
        adr 	x3, hyp_traps_vector
	b start_hypervisor

	nop
	.size _start, . - _start

        .global _hang
_hang:
        wfi
        b _hang

	.section .bss
dummy_stack_end:
	.balign	32
	.skip	4 * 1024
dummy_stack_top:

