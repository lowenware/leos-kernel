ENTRY(_start)
SECTIONS
{
    . = 0xfffffff000000000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    . = ALIGN(0x1000);

    LD_RODATA_BASE = .;
    .rodata : { *(.rodata) }
    . = ALIGN(0x1000);

    LD_DATA_BASE = .;
    .data : { *(.data) }
    . = ALIGN(8);

    LD_BSS_BASE = .;
    .bss : { *(.bss) }
    . = ALIGN(0x1000);

    . = . + 0x1000;
    LD_STACK_PTR = .;

    LD_TTBR1_BASE = .;
    . = . + 0x1000;

    LD_TTL2_BASE = .;
    . = . + 0x1000;

    LD_TTL3_BASE = .;

    . = . + 0x1000;
    LD_TTBR0_BASE = .;

    . = . + 0x1000;
    LD_KERNEL_END = .;
}
