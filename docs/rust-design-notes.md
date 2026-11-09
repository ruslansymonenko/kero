# Rust design notes

These are starting recommendations, not decisions about every later machine detail.

## Virtual memory and host memory

Think of virtual RAM as a numbered row of boxes inside a Rust collection. The number on a box is a *virtual address*. Rust's own memory addresses are an implementation detail and should not be exposed to simulated programs. Use safe collection access and a clear error for an invalid virtual address. A `u8` cell accepts only 0–255 by type, but the address can still be outside RAM, and arithmetic can still overflow.

At first, keep one flat RAM region. Let the loader validate a complete range before writing. Later, draw code, stack, data, and device regions on a memory map before implementing them. A memory-mapped device can be explored after ordinary read/write and CPU execution are working: an address range represents a device register instead of ordinary RAM. Keep device effects behind a bus or memory-access boundary only when that feature is introduced. Virtual protection and process isolation come much later.

Use explicit arithmetic behavior for the virtual CPU: `wrapping_add` for a wrapping 8-bit result, `checked_add` for a possible failure, or `overflowing_add` when a carry flag matters. The instruction contract chooses which applies. Do not let debug and release build behavior define the machine accidentally.

## Ownership and errors

Begin with one owner for each stateful part: `Computer` owns RAM and CPU; methods borrow them as needed. Prefer short, ordered operations over storing long-lived references between modules. Immutable slices or snapshots can expose observation without giving callers write access. Use `Result` for invalid programs, addresses, and device operations the caller can report. `Option` is suitable when absence is normal. A panic is for a broken simulator invariant, not a malformed guest program.

## Output and input path

1. **Terminal observation:** print compact RAM and CPU state while the early machine grows. A trace of one instruction at a time is useful for learning; it can be optional so normal runs stay readable.
2. **Virtual text output:** let a guest program request output through a simple runtime or device interface. The host terminal is an adapter; guest code never calls Rust's `println!` itself.
3. **Framebuffer:** store a small grid of pixel values in a separate module. A pixel's `(x, y)` maps to one collection index. Test coordinates, color representation, and clearing without a window.
4. **Window adapter:** once the framebuffer works, a thin adapter may copy pixels into a host window and translate keyboard events into virtual input. A simple candidate is [`minifb`](https://docs.rs/minifb/latest/minifb/), whose window accepts a 32-bit pixel buffer. Check its current platform support and API at that point, then pin the chosen dependency. The virtual framebuffer must not depend on the window crate.

This order separates the machine's display behavior from how the host shows it. A headless test can then inspect virtual pixels even when no window is available. Avoid a GPU framework until a specific capability calls for it.

## Useful Rust references

- [The Rust Book: ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Book: Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [The `u8` arithmetic methods](https://doc.rust-lang.org/std/primitive.u8.html)
- [Cargo tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Clippy usage](https://doc.rust-lang.org/clippy/usage.html)
