OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(start32)

PAGE_SIZE = 0x1000;
VIRTUAL_BASE = 0xffffffff80000000;

SECTIONS
{
	. = 1M + SIZEOF_HEADERS;

	TEXT_PHYS_BEGIN = .;
	.bootstrap : { *(.bootstrap) }

	. += VIRTUAL_BASE;
	.text : AT(ADDR(.bootstrap) + SIZEOF(.bootstrap)) { *(.text) *(.text.*) }

	data_phys_begin = . - VIRTUAL_BASE;
	.rodata : { *(.rodata) *(.rodata.*) }
	.data : { *(.data) *(.data.*) *(.got) *(.got.*) }
	data_phys_end = . - VIRTUAL_BASE;
	. = ALIGN(PAGE_SIZE);

	bss_phys_begin = . - VIRTUAL_BASE;
	.bss : { *(.bss .bss.*) }
	BSS_PHYS_END = . - VIRTUAL_BASE;
}
