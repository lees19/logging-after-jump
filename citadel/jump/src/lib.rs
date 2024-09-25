#![no_std]
#![no_main]

use stm32f4xx_hal::{ pac, 
    prelude::*}; 

pub fn jump_to_address() -> !{
    let p = pac::Peripherals::take().unwrap(); 

    let gpioc = p.GPIOC.split(); 
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high(); 

    defmt::info!("Turning off LED");
    
    let address = 0x08004000; 

    unsafe { 
        let cp = pac::CorePeripherals::take().unwrap();
        cp.SCB.vtor.write(address as u32);
    }
    
    unsafe{ 
        let jump_address_temp: usize = address + size_of::<u32>();
        let jump_address = *(jump_address_temp as *const usize); 
        let jump = core::mem::transmute::<usize, extern "C" fn() -> !>(jump_address); 
        defmt::info!("Jumping to somewhere else");
        defmt::flush(); 
        // loop{ 
        //     debug::exit(debug::EXIT_SUCCESS); 
        // }
        jump(); 
    }

}