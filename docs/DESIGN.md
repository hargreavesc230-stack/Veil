# Veil Design

## Design philosophy

Veil is designed as a small, high-performance kernel core with privacy as a first-class requirement. The architecture favors explicit boundaries, minimal trusted code, and well-documented invariants.

## Security and privacy principles

- Minimize trusted computing base and isolate privilege.
- Prefer deterministic behavior over implicit or emergent behavior.
- Treat metadata as sensitive and reduce unnecessary exposure.
- Design APIs that make privacy-preserving choices the default.
- Document threat models and assumptions early and often.

## Unsafe Rust policy

Unsafe Rust is minimized, justified, and documented. Each unsafe block must have a clear safety comment, an explicit invariant it relies on, and a reference to the design notes that justify its existence.

## Non-goals

- A full userland or application environment.
- Broad hardware support during early development.
- Compatibility layers for existing OS APIs.
- Optimizing for features that compromise privacy or auditability.
