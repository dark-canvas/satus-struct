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
    memory_type: u8,
    page_num_start: u32,
    page_count: u32,
}

#[repr(C)]
pub struct MemoryMapPage {
    next_memory_map_addr: u64,
    total_pages: usize,
    regions: [MemoryRegion; 340],
}

pub struct MemoryMap {
    raw_data: *mut MemoryMapPage,
}


impl MemoryMap {
    pub fn new_from_page(page_ptr: usize) -> Result<MemoryMap, &'static str> {

        let list = Self::from_page(page_ptr);
        unsafe {
            (*list.raw_data).next_memory_map_addr = 0;
            (*list.raw_data).total_pages = 0;
        }
        Ok(list)
    }

    pub fn from_page(page_ptr: usize) -> Self {
        MemoryMap { raw_data: page_ptr as *mut MemoryMapPage}
    }

    pub fn add_region(&mut self, memory_type: MemoryRegionType, start: Address, end: Address) {
        unsafe {
            let index = (*self.raw_data).total_pages;

            let region = &mut (*self.raw_data).regions[index];
            region.memory_type = memory_type as u8;
            region.page_num_start = (start / 4096).try_into().expect("Start address fits in a u32");
            region.page_count = ((end - start) / 4096).try_into().expect("Page count fits in a u32");

            (*self.raw_data).total_pages = index+1;
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
