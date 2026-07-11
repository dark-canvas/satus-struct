use crate::config_page::ConfigPage;
use crate::types::Address;

const MAX_MODULES: usize = 46; // To fit in a 4096 byte page

pub type ModuleList = ConfigPage<ModuleListConfig>;

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
struct ModuleListConfig {
    header: ModuuleListHeader,
    modules: [ModuleInfo; MAX_MODULES], // If not enough we can link pages somewhow
}

impl ModuleList {

    pub fn get_num_modules(&self) -> usize {
        self.header.num_modules as usize
    }

    pub fn append(&mut self, name: &[u8], base_addr: Address, size: usize, entry: usize) -> Result<(), &'static str> {
        let num_modules = self.header.num_modules as usize;
        if num_modules >= MAX_MODULES {
            return Err("Module list is full");
        }

        let module_info = &mut self.modules[num_modules];
        // Copy the name into the module_info, truncating if necessary
        let copy_len = core::cmp::min(name.len(), 64);
        module_info.module_name[..copy_len].copy_from_slice(&name[..copy_len]);
        module_info.entry = entry;
        module_info.start = base_addr;
        module_info.size = size;

        self.header.num_modules += 1;

        Ok(())
    }

    pub fn get_module_info(&self, index: usize) -> Option<&ModuleInfo> {
        let num_modules = self.header.num_modules as usize;
        if index >= num_modules {
            return None;
        }
        Some(&self.modules[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<ModuuleListHeader>(), 48);
        assert_eq!(std::mem::size_of::<ModuleInfo>(), 88);
        assert_eq!(std::mem::size_of::<ModuleListConfig>(), 4096);
    }
}
