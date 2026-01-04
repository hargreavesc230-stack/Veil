# Veil Roadmap

This roadmap defines the phased milestones for the Veil kernel. Each phase has a clear goal, definition of done, and verification criteria.

## Phase 0 - Repo + Tooling (IN PROGRESS)

**Goal**
- Establish the project foundation, documentation, and development workflow.

**Definition of Done**
- Core documentation exists: `README.md`, `ROADMAP.md`, `CONTRIBUTING.md`, and design notes.
- Repository structure is defined and agreed.
- Basic contribution workflow and commit conventions are documented.

**Test / Verify**
- Documentation reviewed for completeness and consistency.
- Links between docs resolve correctly.
- Phase status and scope are reflected in `ROADMAP.md`.

## Phase 1 - Boot "Hello, Veil" (TODO)

**Goal**
- Produce a minimal bootable kernel that outputs a recognizable "Hello, Veil" message.

**Definition of Done**
- Kernel boots to a known-good entry point.
- A visible "Hello, Veil" message is produced on target output.
- Boot path is documented at a high level.

**Test / Verify**
- Boot in a virtualized x86_64 environment.
- Confirm the output message appears reliably.
- Record the boot path and limitations in docs.

## Phase 2 - IDT/GDT + exceptions (TODO)

**Goal**
- Establish descriptor tables and basic exception handling.

**Definition of Done**
- GDT and IDT are initialized with a minimal, documented layout.
- Common CPU exceptions are handled with readable diagnostics.
- Failure modes are deterministic and documented.

**Test / Verify**
- Trigger representative exceptions and confirm handlers run.
- Verify descriptor tables are loaded and stable.
- Document observed behavior and constraints.

## Phase 3 - Paging + frame allocator (TODO)

**Goal**
- Enable paging and provide a basic physical frame allocator.

**Definition of Done**
- Paging is enabled with a minimal, defined memory map.
- Frame allocator can allocate and free frames safely.
- Memory management invariants are documented.

**Test / Verify**
- Validate paging is active and stable after enablement.
- Exercise allocator with simple allocate/free sequences.
- Record memory map assumptions in docs.

## Phase 4 - Scheduler (TODO)

**Goal**
- Introduce a minimal scheduler for kernel threads or tasks.

**Definition of Done**
- A basic scheduling policy is implemented and documented.
- Context switching is stable for a small set of tasks.
- Scheduling constraints and limitations are recorded.

**Test / Verify**
- Run multiple kernel tasks and observe fair switching.
- Verify tasks yield and resume correctly.
- Document timing and preemption assumptions.

## Phase 5 - Syscalls + userspace ABI stub (TODO)

**Goal**
- Define a minimal syscall interface and a stub userspace ABI.

**Definition of Done**
- Syscall entry/exit path is defined and documented.
- A minimal ABI surface is specified for future userspace.
- Error handling conventions are recorded.

**Test / Verify**
- Invoke a representative syscall path in a controlled test.
- Validate register preservation and return semantics.
- Document ABI contract and versioning plan.

## Phase 6 - Privacy primitives (TODO)

**Goal**
- Establish kernel-level primitives that enforce privacy expectations.

**Definition of Done**
- Privacy goals are formalized as kernel invariants.
- Initial primitives are defined with clear contracts.
- Threat model assumptions are documented.

**Test / Verify**
- Validate invariants with targeted scenarios.
- Review contracts for completeness and clarity.
- Document remaining privacy gaps and next steps.
