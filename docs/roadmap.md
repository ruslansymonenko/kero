# Roadmap

This is a fresh Rust learning path, not a record of Python progress. Finish and review one capability at a time. Each task below names an observable result, the module responsible, the connection to the running application, and completion evidence. Split a task into smaller implementation steps when teaching it, while preserving the complete goal. Record machine choices in `architecture.md` and actual progress in `progress.md`.

Rust can support precise types, explicit errors, fixed-width arithmetic, and clear state ownership. Those tools deepen the model; they do not require real pointers, `unsafe`, threads, or a GUI at the start.

## Stage 1 — A handwritten bytecode program runs

**Milestone:** `cargo run` loads a tiny handwritten byte sequence into virtual RAM, runs it one instruction at a time, shows a resulting value, and halts.

### 1.1 Fixed-size virtual RAM visible in the application

- **Existing path:** `src/main.rs` builds and prints a starter message. No virtual computer exists yet.
- **Idea:** Imagine four numbered boxes, each holding one number from 0 to 255. A box number is an *address*; reading checks its contents and writing replaces them. RAM is this row of boxes.
- **Result:** `cargo run` creates a small RAM, writes a byte, and displays the resulting cells. A new `memory` module owns the cells; `main` creates and observes it. The application is the integration point.
- **Recommended design:** One fixed-length collection of `u8` initialized to zero; controlled read and write operations return clear failures for invalid addresses. Use safe indexing. `u8` prevents a value above 255 from being passed as a `u8`, while address validity still needs a runtime check.
- **Trace:** Four cells start `[0, 0, 0, 0]`; write value `7` at address `2`; observe `[0, 0, 7, 0]`. Address `4` is invalid because the last address is `3`.
- **Acceptance:** First and last addresses work; invalid addresses fail without changing RAM; capacity stays fixed; the application displays a changed cell. An empty RAM may be disallowed with a clear creation error or supported explicitly—choose and document one rule before implementing it.
- **Outside this task:** Programs, loader, CPU, physical pointers, memory-mapped devices, and graphics.
- **First implementation step:** Draw the four boxes and state which addresses exist and what they initially contain. Then create the `memory` module with its initial fixed-size state and make `main` display it. Add write/read behavior in the following steps of this same feature.
- **Rust concepts when needed:** `u8`, `usize`, collections, `struct`, `Result`, visibility, immutable versus mutable borrow.

### 1.2 Load a complete program into RAM

- **Result and owner:** `cargo run` shows sample bytes placed in order at a chosen start address. A `program_loader` module validates and copies a borrowed byte sequence; `memory` still owns the cells. `main` connects them.
- **Trace:** `[0, 0, 0, 0, 0]` plus bytes `[10, 20, 30]` at address `1` becomes `[0, 10, 20, 30, 0]`.
- **Acceptance:** Exact fit at the end works; an oversized or invalid range returns an error before changing any cell; the application shows loaded bytes.
- **Rust focus:** `&[u8]` as read-only program data, checked range arithmetic, error propagation. No instruction decoding yet.

### 1.3 Connect CPU starting state to loading

- **Result and owner:** A `cpu` module holds the next-instruction address (*program counter*) and whether execution has stopped (*halted*). A `computer` module owns RAM and CPU, asks the loader to finish, and only then prepares the CPU at the program's start. `main` shows both states.
- **Trace:** Four zero cells and CPU `(PC 0, ready)` receive bytes `[8, 3]` at address `1`; afterward RAM is `[0, 8, 3, 0]` and PC is `1`. A failed load leaves both states as they were.
- **Acceptance:** New/reset CPU state is predictable; successful load sets PC correctly; failed load preserves RAM and CPU; `cargo run` shows the combined result.
- **Rust focus:** state ownership, small status snapshots, ordered mutable borrows. No instruction execution or registers yet.

### 1.4 Document and represent a tiny instruction contract

- **Result and owner:** A small `instruction` module and table in `architecture.md` define at least `HALT`, putting a byte in a register, and one observable state change. `main` can show a handwritten program and its human-readable intended steps; CPU execution waits for later tasks.
- **Acceptance:** Every opcode has byte value, length, operands, effect, and unknown/truncated behavior. A sample sequence can be decoded by hand.
- **Rust focus:** an `enum` for valid decoded instructions. Keep the first set small; defer arithmetic and branches.

### 1.5 Fetch and decode at the program counter

- **Result and owner:** A decoder reads RAM through its public interface at the CPU's PC and returns one meaningful instruction or a clear error. `Computer` exposes a one-instruction observation in the runnable path.
- **Acceptance:** Valid one-byte and multi-byte instructions decode; invalid opcode and missing operand fail; the byte consumption and next PC rule are documented.
- **Rust focus:** `Result<Instruction, Error>`, matching variants, safe checked access. No effects yet.

### 1.6 Execute one step

- **Result and owner:** CPU execution applies one decoded instruction to its own state or RAM via `Computer`; `cargo run` can show before and after states for a single step.
- **Acceptance:** Each first instruction changes only documented state; `HALT` stops execution; a step after halt has a defined outcome; malformed input does not silently advance state.
- **Rust focus:** `match`, mutable borrowing, explicit state transition. Trace byte reads and PC movement on paper first.

### 1.7 Run a complete tiny program

- **Result and owner:** `Computer` repeatedly steps until `HALT`; `main` loads and runs a small byte sequence, then reports the final register or RAM value.
- **Acceptance:** A short program has the expected final state and explicit halt; malformed code yields a useful error; a step limit prevents an accidental endless run during development.
- **Rust focus:** execution loop and error propagation. The loop is single-threaded.

## Stage 2 — A CPU that calculates and chooses

### 2.1 Arithmetic with defined byte behavior

- **Result:** A program computes a value and exposes a status flag. CPU owns the register and flag; `Computer` and `main` show the result.
- **Acceptance:** Normal and boundary arithmetic follow a documented wrap or overflow rule, including flag changes. Use explicit `u8` arithmetic methods so build mode cannot alter guest behavior.

### 2.2 Branching and a terminating loop

- **Result:** Bytecode can choose another instruction address and repeat work. CPU changes PC; the decoder and RAM still define instruction boundaries; `main` runs a small terminating loop.
- **Acceptance:** Taken and untaken branches, a backward jump, and invalid targets behave as specified. Decide absolute or relative addresses after tracing a sample.

### 2.3 Stack and a subroutine call

- **Result:** A program temporarily saves a byte and calls a reusable instruction sequence, then returns. CPU owns a stack pointer; RAM stores values; `Computer` runs a demonstration.
- **Acceptance:** Last-in-first-out order, return address restoration, stack bounds, and collision with code/data have explicit behavior. Start with push/pop, then add call/return within the same capability if the task needs splitting.

### 2.4 Inspectable execution trace

- **Result:** A debugging view can show PC, decoded instruction, and changed state for each step of a program. `Computer` emits observations; a terminal adapter prints them.
- **Acceptance:** Trace order matches actual execution and can be disabled for normal runs. This is host observability, not a guest instruction.

## Stage 3 — Write assembly instead of byte values

### 3.1 Assemble a runnable instruction sequence

- **Result:** An `assembler` module turns a small sequence of textual instructions into bytes accepted by the existing loader; `main` assembles and runs it.
- **Acceptance:** Every supported mnemonic works; bad names and operands report useful locations; resulting bytes execute like the handwritten version.

### 3.2 Named jump targets

- **Result:** Assembly labels replace numeric jump addresses in a runnable loop. The assembler resolves names; CPU and loader stay unchanged.
- **Acceptance:** Forward/backward labels work; duplicate or missing names fail clearly; a labeled loop runs correctly. Learn two-pass translation only when this need appears.

### 3.3 Constants and diagnostics

- **Result:** A larger assembly demo uses named constants and readable source-line errors. Assembler owns names and diagnostics; app displays them.
- **Acceptance:** Reuse, redefinition, unknown names, and line numbers behave predictably.

## Stage 4 — A small programming language

### 4.1 Tokenize an expression

- **Result:** A `lexer` turns a tiny source expression into named tokens; `main` shows those tokens so a learner can inspect how source text is divided before parsing exists.
- **Acceptance:** Numbers, names, operators, whitespace, and invalid characters have defined behavior and locations. Keep the first expression grammar very small.

### 4.2 Parse with precedence

- **Result:** A `parser` produces a syntax tree that preserves grouping; the compiler path uses it rather than guessing from token order.
- **Acceptance:** Grouped and ungrouped expressions differ as intended; malformed syntax reports its position. An `enum` can express tree variants.

### 4.3 Compile assignments and expressions

- **Result:** Source assignments compile to assembly or bytecode and run on the existing CPU; `main` shows a computed machine value.
- **Acceptance:** At least one assignment plus expression works end to end; undefined names fail clearly. Compiler owns name-to-storage mapping.

### 4.4 Compile conditions and loops

- **Result:** A source program chooses a branch and repeats work; compiler lowers control flow to existing jumps.
- **Acceptance:** Both branch outcomes and a terminating loop work. Do not add new CPU semantics unless required by the language contract.

### 4.5 Functions and indexed data

- **Result:** Source code calls a function and accesses a small collection, reusing the CPU stack and RAM.
- **Acceptance:** Nested calls return correctly; valid indexes work; out-of-range access has a defined guest behavior. Split this into two capabilities if either deserves its own traced example.

## Stage 5 — Persistent virtual files

### 5.1 Block-backed virtual disk

- **Result:** A `disk` module stores fixed-size virtual blocks in one host file, and a small app command writes, closes, reopens, and reads a block.
- **Acceptance:** Boundary blocks survive restart; invalid block numbers fail; host-file errors remain distinguishable from guest errors. Use standard-library file I/O.

### 5.2 File allocation and metadata

- **Result:** A `filesystem` gives bytes a virtual file identity, using `disk` blocks for contents and metadata; the app creates and reopens a file.
- **Acceptance:** Empty and multi-block files, full disk, reopen, and non-overlapping allocation work. Define an on-disk format before depending on it.

### 5.3 Directories and paths

- **Result:** The app creates, lists, and reads named files through virtual paths. Filesystem resolves names to metadata; host paths remain internal to the disk adapter.
- **Acceptance:** Root and one nested directory, duplicate names, and missing paths behave clearly.

## Stage 6 — A kernel-like runtime

### 6.1 Process state and program launch

- **Result:** A `runtime` loads a program, gives it its own CPU state, runs it, and records completion. `main` launches a guest program through the runtime.
- **Acceptance:** New, runnable, and finished states are distinct; two processes cannot accidentally share CPU state. A guest process is simulated state, not a host OS process.

### 6.2 System calls and text output

- **Result:** A guest requests output through a narrow runtime interface; the runtime sends text to a terminal adapter. CPU executes the request instruction; guest code never calls host APIs directly.
- **Acceptance:** A known request prints the expected text; unknown requests fail predictably; output ordering is observable. File requests can follow once the filesystem is connected.

### 6.3 Sequential round-robin scheduling

- **Result:** Two programs make alternating progress under one simulated CPU, using a fixed number of steps per turn. Runtime owns the queue and resumes saved process state.
- **Acceptance:** Both programs progress and finish; halted programs leave the queue. Use one host thread so scheduling rules stay visible.

### 6.4 Address translation and protection

- **Result:** Each process sees its own allowed address region. Runtime or a memory-access layer translates guest addresses before RAM access.
- **Acceptance:** Valid local addresses reach the intended cells; cross-region access fails; one process cannot corrupt another's region. Begin with base/limit addressing before considering paging.

## Stage 7 — Interaction and visual output

### 7.1 Text shell

- **Result:** A `shell` accepts `ls`, `cd`, `cat`, and a program launch command through runtime/filesystem interfaces.
- **Acceptance:** A manual walkthrough demonstrates all commands; malformed input reports errors without ending the shell. Keep parsing separate from execution.

### 7.2 Virtual framebuffer

- **Result:** Guest actions change a small pixel grid owned by a `framebuffer` module. A terminal or file-based observation can display or inspect it before any window exists.
- **Acceptance:** Corners, interior pixel, clear, and invalid coordinates work; the pixel-to-buffer mapping is documented. Decide pixel format and resolution here, not in Stage 1.

### 7.3 Window and keyboard adapters

- **Result:** A thin host window displays framebuffer pixels; keyboard events travel back through the runtime to the intended guest. The virtual display logic remains usable headlessly.
- **Acceptance:** A demo draws a shape or text and responds to one key; tests cover virtual pixels and input delivery, while a manual check covers the actual window. Evaluate a simple current window crate at this stage.

### 7.4 Optional deeper directions

Choose one after the simpler system works, and first divide it into complete capabilities: memory-mapped I/O, an interrupt/timer model, paging, pipes, richer graphics, or a small debugger. Rust makes these models comfortable to express with explicit types and ownership, but none is required for the initial machine.
