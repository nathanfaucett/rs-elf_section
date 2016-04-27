#![no_std]


pub type ElfHalf = u16;
pub type ElfWord = u32;
pub type ElfXword = u64;


#[cfg(target_pointer_width = "32")]
pub type ElfAddress = u32;
#[cfg(target_pointer_width = "64")]
pub type ElfAddress = u64;

#[cfg(target_pointer_width = "32")]
pub type ElfOffset = u32;
#[cfg(target_pointer_width = "64")]
pub type ElfOffset = u64;


#[macro_use]
extern crate bitflags;


#[derive(Debug)]
#[repr(C)]
pub struct ElfSection {
    name: ElfWord,
    typ: ElfWord,
    flags: ElfOffset,
    address: ElfAddress,
    offset: ElfOffset,
    size: ElfOffset,
    link: ElfWord,
    info: ElfWord,
    address_align: ElfOffset,
    entry_size: ElfOffset,
}

impl ElfSection {
    pub fn new(
        name: ElfWord,
        typ: ElfWord,
        flags: ElfOffset,
        address: ElfAddress,
        offset: ElfOffset,
        size: ElfOffset,
        link: ElfWord,
        info: ElfWord,
        address_align: ElfOffset,
        entry_size: ElfOffset,
    ) -> Self {
        ElfSection {
            name: name,
            typ: typ,
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

    pub fn get_name(&self) -> ElfWord { self.name }
    pub fn get_type(&self) -> ElfWord { self.typ }
    pub fn get_address(&self) -> ElfAddress { self.address }
    pub fn get_offset(&self) -> ElfOffset { self.offset }
    pub fn get_size(&self) -> ElfOffset { self.size }
    pub fn get_link(&self) -> ElfWord { self.link }
    pub fn get_info(&self) -> ElfWord { self.info }
    pub fn get_address_align(&self) -> ElfOffset { self.address_align }
    pub fn get_entry_size(&self) -> ElfOffset { self.entry_size }

    pub fn get_start_address(&self) -> usize {
        self.address as usize
    }
    pub fn get_end_address(&self) -> usize {
        (self.address + self.size) as usize
    }
    pub fn get_flags(&self) -> ElfSectionFlags {
        ElfSectionFlags::from_bits_truncate(self.flags)
    }
    pub fn is_allocated(&self) -> bool {
        self.get_flags().contains(ELF_SECTION_ALLOCATED)
    }
}

#[repr(u32)]
pub enum ElfSectionType {
    Unused = 0,
    ProgramSection = 1,
    LinkerSymbolTable = 2,
    StringTable = 3,
    RelaRelocation = 4,
    SymbolHashTable = 5,
    DynamicLinkingTable = 6,
    Note = 7,
    Uninitialized = 8,
    RelRelocation = 9,
    Reserved = 10,
    DynamicLoaderSymbolTable = 11,
    // plus environment-specific use from 0x60000000 to 0x6FFFFFFF
    // plus processor-specific use from 0x70000000 to 0x7FFFFFFF
}

#[derive(Clone)]
pub struct ElfSectionIter {
    current_section: &'static ElfSection,
    remaining_sections: u32,
    entry_size: u32,
}

impl ElfSectionIter {
    pub fn new(
        current_section: &'static ElfSection,
        remaining_sections: u32,
        entry_size: u32,
    ) -> Self {
        ElfSectionIter {
            current_section: current_section,
            remaining_sections: remaining_sections,
            entry_size: entry_size,
        }
    }
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSection;
    fn next(&mut self) -> Option<&'static ElfSection> {
        if self.remaining_sections == 0 {
            None
        } else {
            let section = self.current_section;
            let next_section_addr = (self.current_section as *const _ as u32) + self.entry_size;
            self.current_section = unsafe{ &*(next_section_addr as *const ElfSection) };
            self.remaining_sections -= 1;
			if section.get_type() == ElfSectionType::Unused as u32 {
				self.next()
			} else {
	            Some(section)
			}
        }
    }
}

bitflags! {
    pub flags ElfSectionFlags: ::ElfAddress {
        const ELF_SECTION_WRITABLE = 0x1,
        const ELF_SECTION_ALLOCATED = 0x2,
        const ELF_SECTION_EXECUTABLE = 0x4,
        // plus environment-specific use at 0x0F000000
        // plus processor-specific use at 0xF0000000
    }
}
