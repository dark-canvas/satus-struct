use crate::types::Address;

const MAX_MODULES: usize = 46; // To fit in a 4096 byte page

// 16 bytes of header
#[repr(C)]
struct ModuuleListHeader {
    version: u16,
    num_modules: u16,
    reserved: [u8; 44], // Padding 

}
// Create an array of these that ends up being 4096 bytes (a page)
// KernelModuuleInfo is 64 bytes, so we can fit 64 of them in a page
// The location of the module is listed in page granularity, using u16 
// types, which can span ~2GB of memory, which should more more than 
// enough for bood-loader-loaded modules (other modules can be loaded 
// later, utilizing these other modules as needed)
#[repr(C)]
pub struct ModuleInfo {
    module_name: [u8; 64], // Assuming max module name length of 64 bytes
    entry: usize,
    start: Address,
    size: usize,
}

impl ModuleInfo {
    pub fn get_start_address(&self) -> Address {
        self.start
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

// List of modules loaded, occupies a full page.  This page will be 
// passed to the kernel via a register (rax?)
#[repr(C)]
struct ModuleListPage {
    header: ModuuleListHeader,
    modules: [ModuleInfo; MAX_MODULES], // If not enough we can link pages somewhow
}

pub struct ModuleList {
    raw_data: *mut ModuleListPage,
}

impl ModuleList {
    pub fn new_from_page(page_ptr: Address) -> Result<ModuleList, &'static str> {

        let list = Self::from_page(page_ptr);
        unsafe {
            (*list.raw_data).header.version = 1;
            (*list.raw_data).header.num_modules = 0;
        }
        Ok(list)
    }

    pub fn from_page(page_ptr: Address) -> Self {
        ModuleList { raw_data: page_ptr as *mut ModuleListPage}
    }

    pub fn get_page_ptr(&self) -> Address {
        self.raw_data as Address
    }

    pub fn get_num_modules(&self) -> usize {
        unsafe {
            (*self.raw_data).header.num_modules as usize
        }
    }

    pub fn append(&mut self, name: &[u8], base_addr: Address, size: usize, entry: usize) -> Result<(), &'static str> {
        unsafe {
            let num_modules = (*self.raw_data).header.num_modules as usize;
            if num_modules >= 51 {
                return Err("Module list is full");
            }

            let module_info = &mut (*self.raw_data).modules[num_modules];
            // Copy the name into the module_info, truncating if necessary
            let copy_len = core::cmp::min(name.len(), 64);
            module_info.module_name[..copy_len].copy_from_slice(&name[..copy_len]);
            module_info.entry = entry;
            module_info.start = base_addr;
            module_info.size = size;

            (*self.raw_data).header.num_modules += 1;
        }

        Ok(())
    }

    pub fn get_module_info(&self, index: usize) -> Option<&ModuleInfo> {
        unsafe {
            let num_modules = (*self.raw_data).header.num_modules as usize;
            if index >= num_modules {
                return None;
            }
            Some(&(*self.raw_data).modules[index])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<ModuuleListHeader>(), 48);
        assert_eq!(std::mem::size_of::<ModuleInfo>(), 88);
        assert_eq!(std::mem::size_of::<ModuleListPage>(), 4096);
    }
}
