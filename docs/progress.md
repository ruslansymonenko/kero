# Progress journal

- **Current stage:** Stage 1 — build and run the first bytecode program.
- **Current task:** 1.1 — fixed-size virtual RAM visible through the runnable application.
- **Completed tasks:** None. The Cargo starter is setup, not an exercise.
- **Verified:** After the Kero rename, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` (0 tests) pass; `cargo run` prints `Kero: ready for the first exercise`. No simulator behavior exists or has been verified.
- **Decisions:** The project and Cargo package are named Kero; a frog is the mascot and a visual theme, without defining machine behavior. Rust edition 2024; stable toolchain; standard library for educational components; virtual byte values use `u8`; simulator errors should be explicit and recoverable where the caller can handle them. Machine size, address width, instruction encoding, and display format remain open until their tasks.
- **Open issues:** None known. No Python code or progress is inherited.
- **Next action:** Ask Codex to present task 1.1 as a complete capability, then implement its first small step yourself.

Update this journal after meaningful progress. Record actual checks, current behavior, unresolved questions, and the next step. Do not mark a task complete on the basis of an empty-project check.
