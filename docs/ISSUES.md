# Veil Issues

This document maps roadmap phases to issue-like items for planning and tracking.

## Phase 0 - Repo + Tooling

- Define and publish the initial documentation set.
- Establish repository structure and contribution workflow.
- Record the roadmap phases and statuses.

## Phase 1 - Boot "Hello, Veil"

- Produce a minimal bootable kernel entry point.
- Display a recognizable "Hello, Veil" output.
- Document the boot path at a high level.

## Phase 2 - IDT/GDT + exceptions

- Initialize descriptor tables with a documented layout.
- Add basic exception handling with clear diagnostics.
- Record expected exception behavior and limits.

## Phase 3 - Paging + frame allocator

- Enable paging with a defined memory map.
- Provide a simple physical frame allocator.
- Document memory management invariants.

## Phase 4 - Scheduler

- Introduce a minimal scheduling policy.
- Support context switching between kernel tasks.
- Document scheduling constraints and expectations.

## Phase 5 - Syscalls + userspace ABI stub

- Define a minimal syscall surface and entry path.
- Specify a stub userspace ABI contract.
- Record error handling and return conventions.

## Phase 6 - Privacy primitives

- Formalize privacy goals as kernel invariants.
- Define initial privacy-preserving primitives.
- Document threat model assumptions and gaps.
