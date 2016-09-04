use elf_section::ElfSection;
use elf_section_type::ElfSectionType;


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
