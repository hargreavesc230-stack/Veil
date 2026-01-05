/* Minimal linker script for a freestanding x86_64 kernel.
 *
 * This defines the entry symbol and section layout expected by the Rust crate.
 */

ENTRY(_start)

/* Load at 1 MiB physical to keep early bring-up simple. */
KERNEL_START = 0x00100000;

SECTIONS
{
    . = KERNEL_START;

    /* Multiboot2 header must appear early and be 8-byte aligned. */
    .multiboot2 ALIGN(8) : {
        KEEP(*(.multiboot2))
    }

    /* Keep the entry symbol and align to page size for future paging. */
    .text ALIGN(4K) : {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    /* Read-only data aligned for page-granularity mappings. */
    .rodata ALIGN(4K) : {
        *(.rodata .rodata.*)
    }

    /* Writable data aligned to page boundaries. */
    .data ALIGN(4K) : {
        *(.data .data.*)
    }

    /* Zero-initialized data; aligned for clean section boundaries. */
    .bss ALIGN(4K) : {
        *(COMMON)
        *(.bss .bss.*)
    }
}
