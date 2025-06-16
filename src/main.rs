//! This is the main entry point for the RustyStation emulator.
mod cpu;
mod bus; // Tell Rust about our new bus module.

use cpu::Cpu;
use bus::Bus; // Bring the Bus struct into scope.

fn main() {
    println!("Starting RustyStation!");

    // Create a new Bus instance, loading the BIOS file.
    let bus = Bus::new("SCPH1001.BIN");

    // Create the CPU.
    let mut cpu = Cpu::new();
    
    // The main emulator loop.
    let mut i = 0;
    loop {
        println!("\n--- Cycle {} ---", i);

        // Run one instruction cycle, passing a reference to the bus.
        cpu.run_next_instruction(&bus);
        
        // Break the loop after 5 cycles.
        if i >= 4 {
            break;
        }
        i += 1;
    }
    
    println!("\nEmulation finished after 5 cycles.");
    println!("Final CPU state: {:?}", cpu);
}
