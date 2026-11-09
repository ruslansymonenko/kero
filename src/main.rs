mod memory;

use std::println;

use memory::create_memory;

use crate::memory::Memory;

fn main() {
    let mut app_memory: Memory = create_memory();

    match app_memory.write(2, 7) {
        Ok(()) => println!("Succes!"),
        Err(e) => println!("Error: {}", e),
    }

    match app_memory.read(2) {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match app_memory.write(0, 10) {
        Ok(()) => println!("Succes!"),
        Err(e) => println!("Error: {}", e),
    }

    match app_memory.read(0) {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match app_memory.write(3, 15) {
        Ok(()) => println!("Succes!"),
        Err(e) => println!("Error: {}", e),
    }

    match app_memory.read(3) {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    println!("{:?}", app_memory);

    match app_memory.write(4, 7) {
        Ok(()) => println!("Succes!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("{:?}", app_memory);

    match app_memory.read(4) {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
