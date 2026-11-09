# Architecture

This program **simulates** a computer inside an ordinary Rust application. It does not boot hardware or access physical memory. A virtual address is interpreted by the simulator, not dereferenced as a host pointer.

```text
shell / user program / optional window          (later)
                   | requests and display
runtime / processes / simulated devices         (later)
                   | loads and runs programs
virtual disk and filesystem                     (later)
                   | stores bytes and names
tiny language -> assembler -> bytecode          (later)
                                 |
app -> computer -> virtual CPU <-> virtual RAM  (first milestone)
```

The arrows show planned relationships, not existing code. Initially only the Cargo entry point exists. The first milestone is a small handwritten bytecode program that loads into RAM, executes one instruction at a time, changes observable state, and halts.

## Early boundaries

- **RAM** owns a fixed number of `u8` cells initialized to zero. It checks virtual addresses at runtime and offers controlled reads, writes, and a read-only view or snapshot for observation. A host collection's capacity and a cell's 0–255 value range are different ideas.
- **Program** holds bytecode and its intended start address. It is data, not an executing object.
- **Loader** checks that the entire program fits before writing any byte. It does not interpret opcodes or own the CPU.
- **Computer** coordinates RAM, loader, CPU, and eventually devices. A failed load leaves both RAM and CPU state unchanged.
- **CPU** owns the program counter, halted state, and later registers and flags. It reads instructions through the RAM interface and changes state according to the instruction set.
- **Instruction contract** documents opcode bytes, operands, lengths, effects, and invalid/truncated behavior before programs depend on it.

A tiny 8-bit machine is the starting direction. `u8` describes a byte's value, but it does **not** automatically choose RAM capacity or virtual address width. Start with a small RAM and an address type simple enough for indexing. Decide and record exact sizes during Stage 1. Convert virtual addresses to host collection indices at a controlled boundary, and check validity there. Rust ownership protects the simulator's host data; the virtual machine's access rules remain our responsibility.

## Later boundaries

- **Assembler:** handwritten translation from instruction text and labels to bytecode.
- **Language compiler:** handwritten lexer, parser, and code generator for a deliberately small language.
- **Virtual disk/filesystem:** a host file contains virtual blocks, metadata, and named files. Host paths and virtual paths are separate.
- **Runtime:** process state, sequential scheduling, system calls, and simple protection rules. A virtual process is simulated state, not a host OS process.
- **Display and input:** a virtual framebuffer stores pixels; a host renderer shows a copy or view of those pixels. Keyboard events become virtual input events through the runtime.

Keep these as boundaries, not prebuilt frameworks. Add a module only when a task needs it. Update this document with concrete machine choices when the learner reaches them. See [Rust design notes](rust-design-notes.md) for memory, error, and display details.
