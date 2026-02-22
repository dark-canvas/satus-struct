
// TODO: need a nice way to share these with the x86_64 based kernel, without pulling in all the 
// UEFI stuff (shared crate containing only data structs?)
pub struct Config {
    // hide the un-safe-ness
    raw_data: *mut ConfigPage,
}

#[repr(C)]
pub struct ConfigPage {
    pub framebuffer_addr: usize,
    pub framebuffer_size: u32,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_red_mask: u32,
    pub framebuffer_green_mask: u32,
    pub framebuffer_blue_mask: u32,
    pub framebuffer_bytes_per_line: u32,
    // I don't understand why UEFI doesn't have a bytes_per_pixel!?

    pub module_list_addr: usize,
}

impl Config {
    pub fn new_from_page(page_ptr: usize) -> Result<Config, &'static str> {

        let config = Config { raw_data: page_ptr as *mut ConfigPage};
        unsafe {
            // clear the entire page to zero to start with
            core::ptr::write_bytes(config.raw_data as *mut u8, 0, 4096);
        }
        Ok(config)
    }

    pub fn get_page_ptr(&self) -> usize {
        self.raw_data as usize
    }

    pub fn set_module_list(&mut self, module_list_addr: usize) {
        unsafe {
            (*self.raw_data).module_list_addr = module_list_addr;
        }
    }
}