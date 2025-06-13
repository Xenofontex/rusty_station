//! This is the main entry point for the RustyStation emulator.

// This line declares that we have a module named `cpu`
// and tells Rust to look for a file named `cpu.rs` or `cpu/mod.rs`.
mod cpu;

// This brings the `Cpu` struct from our `cpu` module into the current scope,
// so we can use `Cpu` directly instead of the full `cpu::Cpu`.
use cpu::Cpu;

fn main() {
    println!("Starting RustyStation!");

    let mut cpu = Cpu::new();
    
    // This is the main emulator loop. It will run forever,
    // simulating the continuous operation of the CPU.
    // For now, we'll add a counter to stop it after a few cycles.
    let mut i = 0;
    loop {
        println!("\n--- Cycle {} ---", i);

        // Tell the CPU to run one full instruction cycle.
        cpu.run_next_instruction();
        
        // Break the loop after 5 cycles so we don't run forever.
        if i >= 4 {
            break;
        }
        i += 1;
    }
    
    println!("\nEmulation finished after 5 cycles.");
    println!("Final CPU state: {:?}", cpu);
}

