use std::println;

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::program_loader;

pub struct Computer {
    memory: Memory,
    cpu: CPU,
}

impl Computer {
    pub fn new(memory: Memory, cpu: CPU) -> Self {
        Self {
            memory: memory,
            cpu: cpu,
        }
    }

    pub fn status(&self) {
        println!("CPU status: {:?}", self.cpu);
        println!("Memory status: {:?}", self.memory);
    }

    pub fn handle_program(&mut self, address: usize, program: &[u8]) {
        match program_loader::load_program(address, program, &mut self.memory) {
            Ok(()) => {
                println!("Program loaded");

                self.cpu.set_program_counter(address, &self.memory);

                self.status();
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
