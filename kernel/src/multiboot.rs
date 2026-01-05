//! Multiboot2 header definition.
//!
//! The header must be present within the first 32 KiB of the kernel image and
//! aligned to 8 bytes. GRUB scans the early bytes of the binary for this
//! structure to decide whether the kernel is Multiboot2-compliant.

use core::mem::size_of;

/// Multiboot2 magic value.
///
/// This identifies the header to the bootloader.
const MULTIBOOT2_MAGIC: u32 = 0xE852_50D6;

/// Multiboot2 architecture field for i386/x86.
///
/// Multiboot2 defines `0` for i386. GRUB accepts this for x86_64 kernels.
const MULTIBOOT2_ARCH: u32 = 0;

/// End tag type for a Multiboot2 header.
const TAG_TYPE_END: u16 = 0;

/// Multiboot2 header tag flags are unused for the end tag.
const TAG_FLAGS_END: u16 = 0;

/// Multiboot2 end tag size in bytes.
const TAG_SIZE_END: u32 = 8;

/// Multiboot2 header.
///
/// The checksum makes the 32-bit sum of the first three fields equal zero.
#[repr(C)]
struct Multiboot2Header {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    end_tag: Multiboot2EndTag,
}

/// Multiboot2 end tag.
///
/// A minimal header must include this 8-byte terminator.
#[repr(C)]
struct Multiboot2EndTag {
    tag_type: u16,
    flags: u16,
    size: u32,
}

const HEADER_LENGTH: u32 = size_of::<Multiboot2Header>() as u32;
const CHECKSUM: u32 = 0u32.wrapping_sub(
    MULTIBOOT2_MAGIC
        .wrapping_add(MULTIBOOT2_ARCH)
        .wrapping_add(HEADER_LENGTH),
);

/// Multiboot2 header placed in a dedicated, kept section.
///
/// The section ordering in `linker.ld` ensures this lands early in the image.
#[used]
#[link_section = ".multiboot2"]
static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
    magic: MULTIBOOT2_MAGIC,
    architecture: MULTIBOOT2_ARCH,
    header_length: HEADER_LENGTH,
    checksum: CHECKSUM,
    end_tag: Multiboot2EndTag {
        tag_type: TAG_TYPE_END,
        flags: TAG_FLAGS_END,
        size: TAG_SIZE_END,
    },
};
