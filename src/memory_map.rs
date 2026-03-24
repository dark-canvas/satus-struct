use crate::types::Address;

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
pub struct MemoryMapPage {
    next_memory_map_addr: Address,
    num_regions: usize,
    regions: [MemoryRegion; 340],
}

pub struct MemoryMap {
    raw_data: *mut MemoryMapPage,
}


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
    pub fn new_from_page(page_ptr: Address) -> Result<MemoryMap, &'static str> {

        let list = Self::from_page(page_ptr);
        unsafe {
            (*list.raw_data).next_memory_map_addr = 0;
            (*list.raw_data).num_regions = 0;
        }
        Ok(list)
    }

    pub fn from_page(page_ptr: Address) -> Self {
        MemoryMap { raw_data: page_ptr as *mut MemoryMapPage}
    }

    pub fn get_page_ptr(&self) -> Address {
        self.raw_data as Address
    }

    pub fn add_region(&mut self, memory_type: MemoryRegionType, start: Address, end: Address) {
        unsafe {
            let index = (*self.raw_data).num_regions;

            let region = &mut (*self.raw_data).regions[index];
            region.memory_type = memory_type;
            region.page_num_start = (start / 4096).try_into().expect("Start address fits in a u32");
            region.page_count = ((end - start) / 4096).try_into().expect("Page count fits in a u32");

            (*self.raw_data).num_regions = index+1;
        }
    }

    pub fn get_num_regions(&self) -> usize {
        unsafe {
            (*self.raw_data).num_regions
        }
    }

    pub fn get_memory_region(&self, index: usize) -> Option<&MemoryRegion> {
        unsafe {
            let num_regions = (*self.raw_data).num_regions;
            if index >= num_regions {
                return None;
            }
            Some(&(*self.raw_data).regions[index])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<MemoryRegion>(), 12);
        assert_eq!(std::mem::size_of::<MemoryMapPage>(), 4096);
    }
}
