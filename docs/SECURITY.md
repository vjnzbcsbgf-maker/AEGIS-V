# Security engineering policy

AEGIS-V is an experimental research project. No release should be described as secure, hardened, production-ready, or superior to seL4 without evidence.

## Non-negotiable engineering rules

1. Keep `unsafe` Rust forbidden in workspace crates unless a narrowly scoped, reviewed exception is proposed and documented.
2. Never treat guest-provided integers as host pointers.
3. Validate instruction streams before execution or translation.
4. Capability derivation must never increase rights or widen bounds.
5. Revocation and generation handling must be tested for stale-handle behavior.
6. Do not emit executable code until the verifier/JIT contract and memory-protection design are reviewed.
7. Separate guest data from host control structures.
8. Use established, audited cryptographic libraries; do not invent primitives.
9. Treat host compromise as outside the protection offered by an in-process VM model.
10. Document every security invariant and its tests.

## JIT-specific requirements (future)

- W^X policy for generated code.
- No writable alias to executable pages unless the design and threat model explicitly justify it.
- Validate control-flow targets and instruction boundaries.
- Preserve capability checks through optimization.
- Avoid guest-controlled absolute host addresses.
- Define cache invalidation and code lifetime rules.
- Fuzz decoder, verifier, optimizer, and code emitter independently.

## Current limitations

The capability table and per-VM memory are prototypes inside one Rust process. They do not establish isolation against memory corruption in the host, malicious kernel modules, DMA, side channels, or a compromised host OS. No cryptographic memory protection or formal proof is provided.
