
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

    pub fn set_framebuffer_info(&mut self, addr: usize, size: u32, width: u16, height: u16, red_mask: u32, green_mask: u32, blue_mask: u32, bytes_per_line: u32) {
        unsafe {
            (*self.raw_data).framebuffer_addr = addr;
            (*self.raw_data).framebuffer_size = size;
            (*self.raw_data).framebuffer_width = width;
            (*self.raw_data).framebuffer_height = height;
            (*self.raw_data).framebuffer_red_mask = red_mask;
            (*self.raw_data).framebuffer_green_mask = green_mask;
            (*self.raw_data).framebuffer_blue_mask = blue_mask;
            (*self.raw_data).framebuffer_bytes_per_line = bytes_per_line;
        }
    }

    pub fn set_framebuffer(&mut self, addr: usize, size: u32) {
        unsafe {
            (*self.raw_data).framebuffer_addr = addr;
            (*self.raw_data).framebuffer_size = size;
        }
    }

    pub fn set_framebuffer_dimensions(&mut self, width: u16, height: u16, bytes_per_line: u32) {
        unsafe {
            (*self.raw_data).framebuffer_width = width;
            (*self.raw_data).framebuffer_height = height;
            (*self.raw_data).framebuffer_bytes_per_line = bytes_per_line;
        }
    }

    pub fn set_framebuffer_color_masks(&mut self, red_mask: u32, green_mask: u32, blue_mask: u32) {
        unsafe {
            (*self.raw_data).framebuffer_red_mask = red_mask;
            (*self.raw_data).framebuffer_green_mask = green_mask;
            (*self.raw_data).framebuffer_blue_mask = blue_mask;
        }
    }
}