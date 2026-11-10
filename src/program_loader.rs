use crate::memory::Memory;

pub fn load_program(address: usize, program: &[u8], memory: &mut Memory) -> Result<(), String> {
    if address >= memory.get_len() {
        return Err(String::from("Invalid memory address"));
    }

    let program_len: usize = program.len();
    let available_memory: usize = memory.get_len() - address;

    if program_len > available_memory {
        return Err(String::from("Not enough memory"));
    }

    let mut write_address = address;

    for item in program {
        memory.write(write_address, *item)?;
        write_address += 1
    }

    Ok(())
}
