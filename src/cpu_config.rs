use crate::config_page::{ConfigPage, ConfigConstructor};
use crate::types::Address;

use core::sync::atomic::{AtomicBool, AtomicU32};

pub type CpuConfig = ConfigPage<CpuConfigDetails>;

/**
 * BSP (Bootstrap Processor): 
 * The "main" CPU core that wakes up first when you turn on the computer. It handles UEFI initialization, 
 * runs the bootloader, allocates memory, and coordinates the startup of the rest of the system.
 * 
 * AP (Application Processor): 
 * All the other "secondary" CPU cores on the chip. They start up in a halted or asleep state and wait for 
 * the BSP to send them commands (or interrupts) to wake up and begin executing code.
 */

#[repr(C)]
pub struct CpuConfigDetails {
    pub num_cpus: u32,
    pub rsdp_address: Address, // the root system description pointer 
    pub trampoline_address: Address,
}

impl ConfigConstructor for CpuConfigDetails {}

impl CpuConfig {
    pub fn get_num_cpus(&self) -> u32 {
        self.num_cpus
    }

    pub fn set_num_cpus(&mut self, num: u32) {
        self.num_cpus = num;
    }
}
