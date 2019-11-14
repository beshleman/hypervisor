MEMORY { ddr (RWX) : ORIGIN = 0x40000000, LENGTH = 0x8000000 }

SECTIONS
{
        ENTRY(_start)

        .vectors :
        {
                KEEP(*(.vectors*))
        } >ddr

        .text :
        {
                _start_text = .;
                *(.text)
                *(.text*)
                . = ALIGN(8);
                _end_text = .;
        } >ddr

        .data :
        {
                _start_data = .;
                *(.data)
                *(.data*)
                . = ALIGN(8);
                _start_end = .;
        } >ddr

        .bss :
        {
                _start_bss = .;
                *(.bss)
                *(.bss*)
                . = ALIGN(8);
                _end_bss = .;
        } >ddr

        .stack : 
        {
                _start_stack = .;
                PROVIDE(start_stack = .);
                /* For now, let's just guarantee 128KB of stack space */
                . += 0x20000;
                . = ALIGN(8);
                _end_stack = .;
                PROVIDE(end_stack = .);
        }

        /* Add 4KB of buffer */
        . += 0x1000;
	PROVIDE(_end = .);
}
