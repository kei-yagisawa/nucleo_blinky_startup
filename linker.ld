MEMORY {
    FLASH : ORIGIN = 0x08000000, LENGTH = 512K
    RAM   : ORIGIN = 0x20000000, LENGTH =  96K
}

ENTRY(Reset);
EXTERN(VECTORS);

SECTIONS {
    .vector_table ORIGIN(FLASH) : {
        LONG(ORIGIN(RAM) + LENGTH(RAM)); /* スタックポインタ */
        KEEP(*(.vector_table.vectors));  /* ベクタテーブル   */
    } > FLASH

    .text : {
        *(.text .text.*);
    } > FLASH

    .rodata : {
        *(.rodata .rodata.*);
    } > FLASH

    .bss (NOLOAD): {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > RAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata)) {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > RAM
    
    _sidata = LOADADDR(.data);

    /DISCARD/ : {
        *(ARM.exidx .ARM.exidx.*);
    }
}
