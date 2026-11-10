mod memory;
mod program_loader;

use std::println;

use memory::create_memory;
use program_loader::load_program;

use crate::memory::Memory;

fn main() {
    let mut app_memory: Memory = create_memory();

    println!("Memory: {:?}", app_memory);

    match load_program(2, &[1, 7], &mut app_memory) {
        Ok(()) => println!("Program loaded"),
        Err(e) => println!("Error: {}", e),
    }

    println!("Memory: {:?}", app_memory);

    match app_memory.read(2) {
        Ok(value) => println!("Memory value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match app_memory.read(3) {
        Ok(value) => println!("Memory value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
