# Progress journal

- **Current stage:** Stage 1 — build and run the first bytecode program.
- **Current task:** 1.2 — load a complete program into RAM.
- **Completed tasks:** 1.1 — fixed-size virtual RAM visible through the runnable application. The Cargo starter was setup, not an exercise.
- **Verified:** `Memory::read` and `Memory::write` check addresses before indexing the private four-byte array. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` (0 tests) pass. `cargo run` reads back values written at addresses 0, 2, and 3; both read and write at address 4 report `Wrong memory address`. RAM displays `Memory { data: [10, 0, 7, 15] }` both before and after the failed write. These are manual application observations; no behavior tests exist yet.
- **Decisions:** The project and Cargo package are named Kero; a frog is the mascot and a visual theme, without defining machine behavior. Rust edition 2024; stable toolchain; standard library for educational components; virtual byte values use `u8`; simulator errors should be explicit and recoverable where the caller can handle them. The task 1.1 prototype has four cells, `usize` addresses 0–3, and no empty-RAM case. Instruction encoding and display format remain open until their tasks.
- **Open issues:** None known. No Python code or progress is inherited.
- **Next action:** Begin task 1.2 by tracing a short byte sequence loaded into the existing four-cell RAM, then connect a `program_loader` module to `Memory` and `main`.

Update this journal after meaningful progress. Record actual checks, current behavior, unresolved questions, and the next step. Do not mark a task complete on the basis of an empty-project check.
