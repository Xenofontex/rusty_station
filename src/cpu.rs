//! This module contains the definition and implementation of the PlayStation's CPU.

// We need to import the Bus to use it as a parameter.
use crate::bus::Bus;

#[derive(Debug)]
pub struct Cpu {
    pub pc: u32,
    regs: [u32; 32],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0xBFC00000, 
            regs: [0; 32], 
        }
    }

    /// Runs the next instruction cycle, using the bus to fetch instructions.
    pub fn run_next_instruction(&mut self, bus: &Bus) {
        // 1. Fetch
        // Get the current PC to fetch from that address.
        let instruction_address = self.pc;

        // Use the bus to read the 32-bit instruction from memory!
        let instruction = bus.read_32(instruction_address);

        // Increment the PC to point to the *next* instruction.
        self.pc = self.pc.wrapping_add(4);
        
        // 2. Decode & 3. Execute
        self.decode_and_execute(instruction);
    }

    /// Decodes the given instruction and executes it.
    fn decode_and_execute(&mut self, instruction: u32) {
        // Now, this will show the REAL instruction value from the BIOS.
        println!("Decoding instruction {:#010X}...", instruction);
    }
}
