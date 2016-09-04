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

    pub fn get_name(&self) -> u32 { self.name }
    pub fn get_kind(&self) -> u32 { self.kind }
    pub fn get_type(&self) -> u32 { self.kind }
    pub fn get_address(&self) -> usize { self.address }
    pub fn get_offset(&self) -> usize { self.offset }
    pub fn get_size(&self) -> usize { self.size }
    pub fn get_link(&self) -> u32 { self.link }
    pub fn get_info(&self) -> u32 { self.info }
    pub fn get_address_align(&self) -> usize { self.address_align }
    pub fn get_entry_size(&self) -> usize { self.entry_size }

    pub fn get_start_address(&self) -> usize {
        self.address
    }
    pub fn get_end_address(&self) -> usize {
        self.address + self.size
    }
    pub fn get_flags(&self) -> ElfSectionFlags {
        ElfSectionFlags::from_bits_truncate(self.flags)
    }
    pub fn is_allocated(&self) -> bool {
        self.get_flags().contains(ELF_SECTION_ALLOCATED)
    }
}
