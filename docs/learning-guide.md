# Learning guide

Work on one roadmap task at a time. Each task should give the virtual computer a recognizable new ability and keep a runnable path through `src/main.rs`. The learner writes the core implementation. Codex explains, reviews, and supplies increasingly concrete help only when requested.

Before each new component, describe the parts already working. Use an ordinary analogy before a technical name, then trace one small example from initial state through action to result. Define the complete task before the first implementation step:

- observable result and a manual run when possible;
- new module's responsibility, its existing collaborators, and where it joins the runnable application;
- one recommended simple design and the reason for it;
- important normal, edge, and failure behavior;
- what the task does not include.

Use simple and concrete language in learner-facing explanations. Do not rely on shorthand such as "runnable path" or "integration point" without explaining what it means in the current code. Introduce a technical term only when it helps, and immediately define it in ordinary words. Describe each next step as a specific edit or action, followed by the exact result the learner should see.

Then guide only the next small step, with an exact expected observation. Follow with the next step after the learner has tried it. A task is complete when the result works, relevant checks pass, and important choices are recorded in `architecture.md` and `progress.md`.

## Rust while building a computer

Learn Rust features when a computer feature needs them. The first byte memory uses `u8` for cell values and a collection for storage; a collection's position is a host index, while a *virtual address* is a number defined by the machine. Later, a loader introduces borrowed read-only bytes and error returns. CPU state introduces a small `struct` or `enum`; decoding introduces an instruction `enum`; execution makes mutable borrowing and explicit state changes useful.

Rust's type system can reject a wrong kind of value at compile time, but it cannot know that address 12 is outside an eight-cell virtual RAM. The simulator still checks runtime values and defines its own errors. When the compiler objects to a borrow, first draw which component owns the data and who needs read or write access. Choose a simple sequence of operations before reaching for `clone`, shared ownership, or interior mutability.

Do not model virtual RAM with real pointers or `unsafe`. A virtual memory cell is an element of a safe Rust collection. A virtual CPU is ordinary program state, not the host processor. Avoid concurrent execution until sequential stepping and scheduling are understood.

## Review rhythm

1. Explain the whole capability and one concrete state trace.
2. Implement one small connected step.
3. Run `cargo check` for quick feedback; use a manual `cargo run` observation as soon as the feature reaches the app.
4. Check important behavior and boundaries with focused tests. Run formatting, Clippy, tests, and the application before declaring completion.
5. Review the result and update the journal. Keep tests in service of the feature, rather than making them standalone assignments.

At a stage checkpoint, run the stage's existing demonstration and briefly record what you learned, what surprised or broke, and what you would reconsider. This is a reflection on completed capabilities, not another exercise. Stop there if the current machine already meets your learning goal; resume with the next stage when ready.

When stuck, ask first for a hint, then implementation direction, pseudocode, partial code, and finally complete code for a specific part if needed. A small experiment is fine when it helps understand the current feature. The roadmap is a guide and can be adjusted as actual implementation teaches more.
