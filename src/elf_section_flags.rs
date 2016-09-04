

bitflags! {
    pub flags ElfSectionFlags: usize {
        const ELF_SECTION_WRITABLE = 0x1,
        const ELF_SECTION_ALLOCATED = 0x2,
        const ELF_SECTION_EXECUTABLE = 0x4,
        // plus environment-specific use at 0x0F000000
        // plus processor-specific use at 0xF0000000
    }
}
