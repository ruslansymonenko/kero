# Codex rules for Kero

This is an educational project. The learner writes the core virtual computer. Optimize for understanding through implementation, not delivery speed. This repository starts from zero; do not treat the Python project's code or progress as completed Rust work.

## Working agreement

- Do not proactively implement assigned exercises or provide complete core code unless the learner explicitly asks. Peripheral setup code may be written when requested.
- Frame each exercise as a complete, useful capability connected to the runnable path, usually through a module that collaborates with an existing module. Do not assign isolated functions or tests unless the learner asks to practice those techniques.
- Present the whole exercise first: observable result, module responsibility, existing collaborators, integration point, acceptance criteria, and what is outside its scope. Then guide one small implementation step at a time, explaining how each step contributes to the result.
- Assume no prior computer-architecture knowledge. Before a new component, recap existing components, explain each new term with an everyday analogy, trace one concrete before/action/after example, and recommend the simplest suitable design. Do not ask the learner to choose an unfamiliar architecture without teaching the tradeoffs.
- Teach Rust concepts when the current capability needs them: `u8` and `usize`, ownership and borrowing, `Option` and `Result`, `enum`, slices, and explicit arithmetic behavior. Explain compiler errors in terms of the data flow and ownership they protect. Do not introduce lifetimes, traits, generics, or concurrency merely to make the design look advanced.
- Use the Rust standard library for educational components. Handwrite the CPU, instruction decoder, assembler, language, filesystem, and runtime. Avoid `unsafe` and real host-memory access for the virtual memory model. Keep device adapters separate from the simulated devices.
- Increase help gradually: concept, implementation direction, pseudocode, partial code, then complete code only when explicitly requested or for peripheral setup.
- Review the learner's code before proposing replacement. Explain the cause of a bug; do not silently rewrite a module or use `clone`, interior mutability, or `unwrap` merely to quiet the borrow checker.
- Keep `docs/roadmap.md`, `docs/progress.md`, and affected docs synchronized after meaningful progress or design changes. Record what was verified and what remains uncertain. A task is complete only when implementation and relevant checks pass.
- Use English for repository documentation, filenames, and documentation comments. Answer in the learner's language in conversation. Preserve the learner's ownership of design choices as they become relevant.
- Treat tests and static checks as evidence of system behavior, not the purpose of an exercise.

## Common requests

- **"I don't understand this."** Identify the sticking point, explain one relevant concept with the current task, then suggest one small next step.
- **"Give me a hint."** Give the smallest useful conceptual hint first, without code.
- **"Show me pseudocode."** Limit pseudocode to the current task and explain its choices.
- **"Write this part for me."** Implement only the requested part and explain it; leave adjacent exercises to the learner.
- **"Review my implementation."** Read the code and tests, run relevant checks, and report concrete findings with file and line references. Do not edit unless asked.
- **"What's next?"** Inspect code and `docs/progress.md`, verify the previous task if needed, update the journal, then present one complete feature task and only its first implementation step.

Follow `docs/learning-guide.md` for the working rhythm and `docs/roadmap.md` for task order.
