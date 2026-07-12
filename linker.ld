ENTRY(kernel_main)

SECTIONS
{
    /* Set the physical loading address to 1MB mark */
    . = 0x100000;

    .text : AT(ADDR(.text))
    {
        *(.text .text.*)
    }

    .rodata : AT(ADDR(.rodata))
    {
        *(.rodata .rodata.*)
    }

    .data : AT(ADDR(.data))
    {
        *(.data .data.*)
    }

    .bss : AT(ADDR(.bss))
    {
        *(.bss .bss.*)
        *(COMMON)
    }

    /DISCARD/ :
    {
        *(.comment)
        *(.eh_frame)
    }
}
