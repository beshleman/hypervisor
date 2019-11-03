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
	ldr	x1, =start_stack
	mov	sp, x1

	/* Pass start, end, and phys offset to Rust */
	/* x0 = start, x1 = end, x2 = offset */
        adr 	x0, _start
        adr 	x1, _end
	mov    x2, x10
	b start_mythril

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

