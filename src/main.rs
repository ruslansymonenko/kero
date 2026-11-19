mod computer;
mod cpu;
mod memory;
mod program_loader;

use crate::{computer::Computer, cpu::CPU, memory::Memory};

fn main() {
    let memory: Memory = Memory::new();
    let cpu: CPU = CPU::new();

    let program: &[u8] = &[1, 7];

    let mut computer: Computer = Computer::new(memory, cpu);

    computer.handle_program(1, program);

    // println!("Memory: {:?}", app_memory);

    // match load_program(2, &[1, 7], &mut app_memory) {
    //     Ok(()) => println!("Program loaded"),
    //     Err(e) => println!("Error: {}", e),
    // }

    // println!("Memory: {:?}", app_memory);

    // match app_memory.read(2) {
    //     Ok(value) => println!("Memory value: {}", value),
    //     Err(e) => println!("Error: {}", e),
    // }

    // match app_memory.read(3) {
    //     Ok(value) => println!("Memory value: {}", value),
    //     Err(e) => println!("Error: {}", e),
    // }
}
