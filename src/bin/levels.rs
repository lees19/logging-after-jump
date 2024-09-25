#![no_main]
#![no_std]

use jump::jump_to_address;

use my_app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    defmt::info!("info");  
    defmt::trace!("trace");
    defmt::debug!("debug"); 

    jump_to_address(); 
}