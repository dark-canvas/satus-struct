use crate::config_page::{ConfigPage, ConfigConstructor};
use crate::types::Address;

pub type MemoryMap = ConfigPage<MemoryMaps>;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum MemoryRegionType {
    Available,
    Allocated,
    Reserved,
    NonExistent,
}

#[repr(C)]
pub struct MemoryRegion {
    memory_type: MemoryRegionType,
    page_num_start: u32,
    page_count: u32,
}

#[repr(C)]
pub struct MemoryMaps {
    next_memory_map_addr: Address,
    num_regions: usize,
    regions: [MemoryRegion; 340],
}

impl ConfigConstructor for MemoryMaps {}

impl MemoryRegion {
    pub fn get_type(&self) -> MemoryRegionType {
        self.memory_type
    }

    pub fn get_start_address(&self) -> Address {
        self.page_num_start as Address * 4096
    }

    pub fn get_end_address(&self) -> Address {
        self.get_start_address() + (self.page_count as Address * 4096)
    }

    pub fn get_address_range(&self) -> (Address, Address) {
        ( self.get_start_address(), self.get_end_address() )
    }
}

impl MemoryMap {

    pub fn add_region(&mut self, memory_type: MemoryRegionType, start: Address, end: Address) {
        let index = self.num_regions;

        let region = &mut self.regions[index];
        region.memory_type = memory_type;
        region.page_num_start = (start / 4096).try_into().expect("Start address fits in a u32");
        region.page_count = ((end - start) / 4096).try_into().expect("Page count fits in a u32");

        self.num_regions = index+1;
    }

    pub fn get_num_regions(&self) -> usize {
        self.num_regions
    }

    pub fn get_memory_region(&self, index: usize) -> Option<&MemoryRegion> {
        let num_regions = self.num_regions;
        if index >= num_regions {
            return None;
        }
        Some(&self.regions[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<MemoryRegion>(), 12);
        assert_eq!(std::mem::size_of::<MemoryMaps>(), 4096);
    }
}
