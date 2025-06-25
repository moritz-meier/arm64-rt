OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")
OUTPUT_ARCH(aarch64)

ENTRY(_start)

INCLUDE memory.ld

SECTIONS {

    . = __TEXT_OFFSET;

    .text ALIGN(0x1000) : AT(ADDR(.text)) {
        __text_start = .;

        *(.text .text.*)

        . = ALIGN(0x8);
        __text_end = .;
    }

    .rodata ALIGN(0x8) : {
        __rodata_start = .;

		*(.rodata .rodata.*)

		. = ALIGN(0x8);
		__rodata_end = .;
    }

    .data ALIGN(0x1000) : {
		__data_start = .;

		*(.data .data.*)

		. = ALIGN(0x8);
		__data_end = .;
	}

    .bss ALIGN(0x8) (NOLOAD) : {
		__bss_start = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(8);
		__bss_end = .;
	}

    .stack ALIGN(0x1000) (NOLOAD) : {
        __stack_start = .;

        . += __NUM_CPU * __STACK_SIZE;

        . = ALIGN(0x8);
        __stack_end = .;

        . = ALIGN(0x1000);
    }
}