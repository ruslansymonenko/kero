#[derive(Debug)]
pub struct Memory {
    data: [u8; 4],
}

impl Memory {
    fn check_address(&self, address: usize) -> Result<(), String> {
        if address >= self.data.len() {
            Err(String::from("Wrong memory address"))
        } else {
            Ok(())
        }
    }

    pub fn read(&self, address: usize) -> Result<u8, String> {
        self.check_address(address)?;

        Ok(self.data[address])
    }

    pub fn write(&mut self, address: usize, value: u8) -> Result<(), String> {
        self.check_address(address)?;

        self.data[address] = value;

        Ok(())
    }
}

pub fn create_memory() -> Memory {
    let memory: Memory = Memory { data: [0, 0, 0, 0] };

    memory
}
