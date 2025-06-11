/// The main entry point for our RustyStation emulator.
///
/// This function will eventually initialize the simulated hardware
/// and start the main CPU execution loop.

fn main() {
    
    let psx_ram_size_bytes = 2 * 1024 * 1024;

    let mut program_counter: u32 = 0xBFC00000;

    println!("Starting RustyStation!");
    
    println!("Configured RAM size: {} bytes", psx_ram_size_bytes);

    println!("Initial CPU Program Counter: {:#X}", program_counter);

    program_counter = program_counter + 4;

    println!("PC after one instruction fetch: {:#X}", program_counter);

    let s1 = String::from("hello");

    let s2 = s1;
    
    println!("s2 now owns the data:{}", s2);

}
