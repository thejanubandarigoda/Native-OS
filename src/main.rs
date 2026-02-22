#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupts;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello Thejanu!");
    println!("AI-OS is fully operational!");
    println!("System Version: {}", 1.0);

    // IDT initialize 
    interrupts::init_idt(); 

    // PICs initialize (Programmable Interrupt Controllers)
    unsafe { interrupts::PICS.lock().initialize() };
    
    // Enable hardware interrupts (Timer, Keyboard, etc.)
    x86_64::instructions::interrupts::enable();

    println!("Hardware Interrupts are active!");
    println!("Type something on your keyboard: \n");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}