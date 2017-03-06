use elf_section_flags::*;


#[derive(Debug)]
#[repr(C)]
pub struct ElfSection {
    name: u32,
    kind: u32,
    flags: usize,
    address: usize,
    offset: usize,
    size: usize,
    link: u32,
    info: u32,
    address_align: usize,
    entry_size: usize,
}

impl ElfSection {
    #[inline(always)]
    pub fn new(
        name: u32,
        kind: u32,
        flags: usize,
        address: usize,
        offset: usize,
        size: usize,
        link: u32,
        info: u32,
        address_align: usize,
        entry_size: usize,
    ) -> Self {
        ElfSection {
            name: name,
            kind: kind,
            flags: flags,
            address: address,
            offset: offset,
            size: size,
            link: link,
            info: info,
            address_align: address_align,
            entry_size: entry_size,
        }
    }
    #[inline(always)]
    pub fn name(&self) -> u32 { self.name }
    #[inline(always)]
    pub fn kind(&self) -> u32 { self.kind }
    #[inline(always)]
    pub fn address(&self) -> usize { self.address }
    #[inline(always)]
    pub fn offset(&self) -> usize { self.offset }
    #[inline(always)]
    pub fn size(&self) -> usize { self.size }
    #[inline(always)]
    pub fn link(&self) -> u32 { self.link }
    #[inline(always)]
    pub fn info(&self) -> u32 { self.info }
    #[inline(always)]
    pub fn address_align(&self) -> usize { self.address_align }
    #[inline(always)]
    pub fn entry_size(&self) -> usize { self.entry_size }

    #[inline(always)]
    pub fn start_address(&self) -> usize {
        self.address
    }
    #[inline(always)]
    pub fn end_address(&self) -> usize {
        self.address + self.size
    }
    #[inline(always)]
    pub fn flags(&self) -> ElfSectionFlags {
        ElfSectionFlags::from_bits_truncate(self.flags)
    }
    #[inline(always)]
    pub fn is_allocated(&self) -> bool {
        self.flags().contains(ELF_SECTION_ALLOCATED)
    }
}
