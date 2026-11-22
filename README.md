# AEGIS-V

**AEGIS-V64** is an experimental 64-bit virtual instruction-set architecture and MultiVM runtime written in Rust. Its design centers on capability-mediated resource access, isolated VM address spaces, and a future AMD64 JIT backend.

> **Security status:** research prototype / scaffolding only. This repository is not a hardened hypervisor, does not provide a hardware security boundary, and must not be used to protect sensitive workloads. Capability checks in this prototype are software mechanisms within one host process. The JIT backend is not implemented yet.

## Project goals

- Define a compact, documented 32-bit instruction encoding for a 64-bit virtual machine.
- Keep the ISA independent of the physical host architecture.
- Make resource authority explicit through unforgeable API-level capability handles.
- Give each MicroVM a separately owned guest-memory buffer and execution context.
- Add a verified translation pipeline before introducing native-code generation.
- Test isolation and capability invariants continuously.

## Current status

| Component | Status |
|---|---|
| ISA word and decoder | Initial scaffold |
| Capability table | Initial safe-Rust prototype |
| Per-VM memory/context | Initial scaffold |
| JIT AMD64 backend | Not implemented |
| Bytecode verifier | Not implemented |
| IPC broker | Not implemented |
| Cryptographic key management | Not implemented |
| Formal verification / security audit | Not started |

## Repository layout

```text
crates/
  aegis-isa/       ISA word format and decoder
  aegis-cap/       Capability table prototype
  aegis-runtime/   MultiVM lifecycle and guest memory
  aegis-jit/       JIT pipeline boundary (backend pending)
docs/
  ISA.md
  SECURITY.md
  THREAT_MODEL.md
```

## Build and test

Requires a stable Rust toolchain (edition 2021).

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Design constraints

1. Guest instructions never directly name host pointers.
2. Guest memory is accessed through VM-owned memory abstractions.
3. Capability derivation may only reduce authority.
4. Revocation invalidates previously issued handles.
5. JIT output must preserve the semantics of the reference ISA implementation.
6. No security claim is accepted without a written threat model and reproducible evidence.

See [docs/ISA.md](docs/ISA.md), [docs/SECURITY.md](docs/SECURITY.md), and [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md).

## License

No license has been selected yet. Until one is added, all rights are reserved by the copyright holder.
