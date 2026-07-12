use core::ops::Deref;
use core::ops::DerefMut;

use crate::types::Address;

/// A struct that acts like the underlying struct T, but is constructed using 
/// a pointer to a page, such that the data can be easily shared between the 
/// bootloader and kernel (or between different address spaces).
pub struct ConfigPage<T> {
    // hide the un-safe-ness
    pub(crate) raw_data: *mut T,
}

impl<T> ConfigPage<T> {
    const _T_LESS_THAN_PAGE: () = {
        if size_of::<T>() <= 4096 {
            panic!("Target struct for ConfigPage must fit into a single page!");
        }
    };

    pub fn new_from_page(page_ptr: Address) -> Result<Self, &'static str> {
        assert!( page_ptr & 4095 == 0, "Address must be page aligned");
        let config = ConfigPage::<T>::from_page(page_ptr);
        unsafe {
            // clear the entire page to zero to start with
            core::ptr::write_bytes(config.raw_data as *mut u8, 0, 4096);
        }
        Ok(config)
    }

    pub fn from_page(page_ptr: Address) -> Self {
        assert!( page_ptr & 4095 == 0, "Address must be page aligned");
        ConfigPage::<T> { raw_data: page_ptr as *mut T}
    }

    pub fn get_page_ptr(&self) -> Address {
        self.raw_data as Address
    }

    pub fn set_page_ptr(&mut self, addr: Address) {
        self.raw_data = addr as *mut T 
    }
}

/// This deref trait hides the hidden/unsafe pointer that's contained in the ConfigPage
impl<T> Deref for ConfigPage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.raw_data)
        }
    }
}

impl<T> DerefMut for ConfigPage<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut (*self.raw_data)
        }
    }
}