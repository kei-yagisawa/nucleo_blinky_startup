MEMORY {
    FLASH : ORIGIN = 0x08000000, LENGTH = 512K
    RAM   : ORIGIN = 0x20000000, LENGTH =  96K
}

ENTRY(Reset);
EXTERN(RESET_VECTOR);

SECTIONS {
    .vector_table ORIGIN(FLASH) : {
        LONG(ORIGIN(RAM) + LENGTH(RAM));        /* スタックポインタ */
        KEEP(*(.vector_table.reset_vector));    /* リセットベクタ   */
    } > FLASH

    .text : {
        *(.text .text.*);
    } > FLASH

    .rodata : {
        *(.rodata .rodata.*);
    } > FLASH

    /DISCARD/ : {
        *(ARM.exidx .ARM.exidx.*);
    }
}