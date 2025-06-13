//! This module contains the definition and implementation of the PlayStation's CPU.

/// Represents the MIPS R3000A CPU of the PlayStation.
///
/// `pub` makes the struct visible to other modules (like main.rs).
#[derive(Debug)]
pub struct Cpu {
    /// Program Counter: Points to the memory address of the next instruction to be executed.
    /// `pub` makes this field accessible from outside the struct, if needed.
    pub pc: u32,

    /// General-Purpose Registers: A set of 32 registers, 32-bits each.
    /// These are kept private (`pub` is omitted) because we want to control access
    /// to them through specific methods later.
    regs: [u32; 32],
}

impl Cpu {
    /// Creates a new `Cpu` instance in its initial power-on state.
    /// `pub` allows us to call this `new` function from main.rs.
    pub fn new() -> Self {
        Cpu {
            // The PlayStation BIOS starts execution at this specific memory address.
            pc: 0xBFC00000, 
            
            // Initialize all 32 registers to 0.
            regs: [0; 32], 
        }
    }

        /// Runs the next instruction cycle (Fetch, Decode, Execute).
    pub fn run_next_instruction(&mut self) {
        // 1. Fetch
        // We don't have memory yet, so we can't actually fetch an instruction.
        // But we can simulate fetching by getting the current PC.
        let instruction_address = self.pc;

        // An essential part of fetching is incrementing the PC to point to the *next* instruction.
        // MIPS instructions are always 4 bytes long.
        self.pc = self.pc.wrapping_add(4);
        
        // 2. Decode & 3. Execute
        // For now, we'll just print a message.
        // In the future, this is where the magic will happen.
        println!("Fetched instruction at address {:#X}, now decoding...", instruction_address);
        
        // We will call another function to handle the actual instruction logic.
        self.decode_and_execute(0x00000000); // Placeholder instruction
    }

    /// Decodes the given instruction and executes it.
    /// `instruction` is the 32-bit value fetched from memory.
    fn decode_and_execute(&mut self, instruction: u32) {
        // TODO: This will become a large `match` statement to identify the instruction.
        println!("Decoding instruction {:#010X} and executing... (Not yet implemented)", instruction);
    }

}
