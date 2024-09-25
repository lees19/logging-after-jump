#![no_main]
#![no_std]

use ums as _; 
use stm32f4xx_hal::{ pac, 
    prelude::*}; 

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    defmt::println!("asdfl;kj"); 
    defmt::info!("Hello, world!");
    defmt::flush(); 

    let p = pac::Peripherals::take().unwrap(); 

    let gpioc = p.GPIOC.split(); 
    let mut led = gpioc.pc13.into_push_pull_output(); 

    defmt::info!("turning on LED"); 
    for _ in 0..300_000{ 
        led.set_low(); 
    }
    led.set_high(); 
    defmt::error!("turning off LED"); 

    ums::exit()
}