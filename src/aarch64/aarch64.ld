SECTIONS
{
    . = 0xffffff8000000000;

    __kernel_base = .;
    .text : {
        *(.text._start)
        *(.text*)
    }

    __rodata_base = .;
    .eh_frame : { *(.eh_frame) }
    .rodata : { *(.rodata*) }
    . = ALIGN(0x1000);

    __data_base = .;
    .data : { *(.data*) }

    .bss ALIGN(8) :
    {
        __bss_base = .;
        *(.bss*)
        __bss_end = .;
    }

    . = ALIGN(0x1000);

    __kernel_end = .;
}
