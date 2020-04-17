ENTRY(_start)
SECTIONS
{
    . = 0xffffff8000000000;
    .text.start : { *(.text.start) }
    . = ALIGN(0x800);
    .text.exceptions : { *(.text.exceptions) }
    .text : { *(.text) }
    . = ALIGN(0x1000);

    LD_RODATA_BASE = .;
    .rodata : { *(.rodata) }
    . = ALIGN(0x1000);

    LD_DATA_BASE = .;
    .data : { *(.data) }

    . = ALIGN(0x0008);

    LD_BSS_BASE = .;
    .bss : { *(.bss) }

    . = ALIGN(0x1000);

    LD_STACK_TOP = .;
    . = . + 0x1000;
    LD_STACK_BOTTOM = .;

    /* start code allocates below
     * - TTBR0_L1
     * - TTBR1_L1,
     * - TTBR1_L2,
     * - TTBR1_L3,
     */
}

