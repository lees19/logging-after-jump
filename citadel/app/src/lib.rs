#![no_main]
#![no_std]

use cortex_m_semihosting::debug; 
use defmt_rtt as _; 
use panic_probe as _;
use stm32f4xx_hal as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
        // cortex_m::asm::nop(); 
    }
}
