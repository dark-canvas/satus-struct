use crate::config_page::ConfigPage;
use crate::types::Address;
use crate::cpu_config::CpuConfig;
use crate::memory_map::MemoryMap;
use crate::module_list::ModuleList;

pub type Config = ConfigPage<BasicConfig>;

#[repr(C)]
pub struct BasicConfig {
    pub framebuffer_addr: Address,
    pub framebuffer_size: u32,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_red_mask: u32,
    pub framebuffer_green_mask: u32,
    pub framebuffer_blue_mask: u32,
    pub framebuffer_bytes_per_line: u32,
    // I don't understand why UEFI doesn't have a bytes_per_pixel!?

    pub cpu_config_addr: Address,

    /// Total number of memory pages in the system
    pub total_pages: u64,
    pub memory_map_addr: Address,

    /// The list of modules loaded into memory by the bootloader
    pub module_list_addr: Address,
}

impl Config {

    pub fn set_cpu_config(&mut self, config: &CpuConfig) {
        self.cpu_config_addr = config.get_page_ptr();
    }

    pub fn set_module_list(&mut self, module_list: &ModuleList) {
        self.module_list_addr = module_list.get_page_ptr();
    }

    pub fn set_memory_map(&mut self, memory_map: &MemoryMap) {
        self.memory_map_addr = memory_map.get_page_ptr();
    }

    pub fn get_cpu_config(&self) -> CpuConfig {
        CpuConfig::from_page(self.cpu_config_addr)
    }

    pub fn get_memory_map(&self) -> MemoryMap {
        MemoryMap::from_page(self.memory_map_addr)
    }

    pub fn get_module_list(&self) -> ModuleList {
        ModuleList::from_page(self.module_list_addr)
    }

    pub fn set_cpu_config_address(&mut self, addr: Address) {
        self.cpu_config_addr = addr;
    }

    pub fn set_module_list_address(&mut self, addr: Address) {
        self.module_list_addr = addr;
    }

    pub fn set_memory_map_address(&mut self, addr: Address) {
        self.memory_map_addr = addr;
    }

    pub fn get_cpu_config_address(&self) -> Address {
        self.cpu_config_addr
    }

    pub fn get_module_list_address(&self) -> Address {
        self.module_list_addr
    }

    pub fn get_memory_map_address(&self) -> Address {
        self.memory_map_addr
    }

    pub fn set_framebuffer_info(&mut self, addr: Address, size: u32, width: u16, height: u16, red_mask: u32, green_mask: u32, blue_mask: u32, bytes_per_line: u32) {
        self.framebuffer_addr = addr;
        self.framebuffer_size = size;
        self.framebuffer_width = width;
        self.framebuffer_height = height;
        self.framebuffer_red_mask = red_mask;
        self.framebuffer_green_mask = green_mask;
        self.framebuffer_blue_mask = blue_mask;
        self.framebuffer_bytes_per_line = bytes_per_line;
    }

    pub fn set_framebuffer(&mut self, addr: Address, size: u32) {
        self.framebuffer_addr = addr;
        self.framebuffer_size = size;
    }

    pub fn set_framebuffer_dimensions(&mut self, width: u16, height: u16, bytes_per_line: u32) {
        self.framebuffer_width = width;
        self.framebuffer_height = height;
        self.framebuffer_bytes_per_line = bytes_per_line;
    }

    pub fn set_framebuffer_color_masks(&mut self, red_mask: u32, green_mask: u32, blue_mask: u32) {
        self.framebuffer_red_mask = red_mask;
        self.framebuffer_green_mask = green_mask;
        self.framebuffer_blue_mask = blue_mask;
    }

    pub fn get_framebuffer_address(&self) -> Address {
        self.framebuffer_addr
    }

    pub fn get_framebuffer_size(&self) -> u32 {
        self.framebuffer_size
    }

    pub fn get_framebuffer_dimensions(&self) -> (u16, u16) {
        (self.framebuffer_width, self.framebuffer_height)
    }

    pub fn get_framebuffer_bytes_per_line(&self) -> u32 {
        self.framebuffer_bytes_per_line
    }
}
