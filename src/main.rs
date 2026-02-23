#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupts;
mod memory; // Link the newly created memory module to the OS

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr; // To use Virtual Addresses

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello Thejanu!");
    println!("AI-OS is fully operational!");
    println!("System Version: {}", 1.0);

    interrupts::init_idt(); 
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("Hardware Interrupts are active!");

    // Print the number of memory regions provided by the bootloader
    println!("BootInfo memory regions: {}", boot_info.memory_map.len());

    // --- Initialize Paging ---
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    // Prepare the Page Table through our new memory.rs
    let _mapper = unsafe { memory::init(phys_mem_offset) };
    
    println!("Memory Paging initialized successfully!");
    println!("Type something on your keyboard: \n");

    loop {}
}

struct Qubit {
    state: &'static str,
}

impl Qubit {
    fn new() -> Self {
        Qubit { state: "0 and 1 simultaneously (Superposition)" }
    }

    fn measure(&self) -> u8 {
        let random_seed = unsafe { core::arch::x86_64::_rdtsc() };
        if random_seed % 2 == 0 { 0 } else { 1 }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}