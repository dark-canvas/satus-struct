#![no_std]

use core::panic::PanicInfo;

#[cfg(test)]
extern crate std;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod config;
pub mod module_list;
