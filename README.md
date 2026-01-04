# Veil Kernel

Veil is a high-performance, privacy-focused operating system kernel written in Rust. It aims to provide a minimal, auditable core with strong security boundaries and privacy-preserving defaults.

Veil follows a roadmap-driven development model. All work is planned and tracked against explicit phases in `ROADMAP.md`.

## Project goals

- High-performance kernel primitives with predictable behavior.
- Privacy-first system design with principled data handling.
- Small, auditable kernel core with clear boundaries.
- Rust-first implementation with explicit handling of unsafe code.
- Initial target architecture: x86_64.

## Non-goals

- A general-purpose OS distribution or userland.
- Feature parity with mature kernels in the short term.
- Hardware driver breadth beyond what is required for bring-up.
- Binary compatibility with existing OS ABIs.

## Roadmap

See `ROADMAP.md` for the phase-based plan and status.
