use crate::memory::Memory;

#[derive(Debug)]
pub struct CPU {
    halted: bool,
    program_counter: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            halted: false,
        }
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn run(&mut self) {
        self.halted = false;
    }

    pub fn set_program_counter(&mut self, count: usize, memory: &Memory) -> Result<(), String> {
        if count > memory.get_len() {
            return Err(String::from("Memory overflow while cpu processing"));
        }

        self.program_counter = count;

        Ok(())
    }
}
