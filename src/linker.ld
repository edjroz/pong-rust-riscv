OUTPUT_ARCH("riscv")
ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    .text : {
        PROVIDE(_text_start = .);
        *(.text.init)
        *(.text .text.*)
        PROVIDE(_text_end = .);
    } > RAM

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } > RAM

    .data : {
        . = ALIGN(4096);
        PROVIDE(_data_start = .);
        *(.sdata .sdata.*)
        *(.data .data.*)
        PROVIDE(_data_end = .);
    } > RAM

    .bss : {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    } > RAM

    PROVIDE(_stack_end = ORIGIN(RAM) + LENGTH(RAM));
}
