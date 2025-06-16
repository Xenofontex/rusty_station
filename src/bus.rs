//! This module contains the memory bus, which handles all memory access.

use std::fs;

// The PlayStation BIOS is 512 KiB in size.
const BIOS_SIZE: usize = 512 * 1024; 

pub struct Bus {
    /// The BIOS ROM, loaded from a file.
    bios: [u8; BIOS_SIZE],
}

impl Bus {
    /// Creates a new Bus instance and loads the BIOS file.
    pub fn new(bios_path: &str) -> Self {
        println!("Loading BIOS from {}...", bios_path);

        let bios_data = fs::read(bios_path)
            .expect("Failed to read BIOS file. Make sure it's in the project root.");
        
        if bios_data.len() != BIOS_SIZE {
            panic!("Invalid BIOS file size. Expected {} bytes, found {} bytes.", 
                BIOS_SIZE, bios_data.len());
        }

        let mut bios = [0; BIOS_SIZE];
        bios.copy_from_slice(&bios_data);

        Bus { bios }
    }

    /// Reads a 32-bit word from the memory bus.
    pub fn read_32(&self, address: u32) -> u32 {
        // For now, we only handle reads from the BIOS region.
        // The BIOS is mapped to address 0xBFC00000.
        if (0xBFC00000..=0xBFC7FFFF).contains(&address) {
            // Calculate the offset from the start of the BIOS.
            let offset = (address - 0xBFC00000) as usize;

            // Read 4 bytes in little-endian format to form a 32-bit word.
            let b1 = self.bios[offset] as u32;
            let b2 = self.bios[offset + 1] as u32;
            let b3 = self.bios[offset + 2] as u32;
            let b4 = self.bios[offset + 3] as u32;

            return b1 | (b2 << 8) | (b3 << 16) | (b4 << 24);
        }

        // If the address is not in a known region, panic for now.
        panic!("Unhandled read from memory address: {:#X}", address);
    }
}
