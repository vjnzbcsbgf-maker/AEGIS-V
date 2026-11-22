# AEGIS-V threat model — draft 0.1

## Assets

- Integrity and confidentiality of each guest's memory and execution state.
- Integrity of capability metadata and VM lifecycle state.
- Correctness of guest-to-host and guest-to-guest interfaces.
- Integrity of translated code and translation metadata.

## Trust boundaries

1. Guest instruction stream to decoder/verifier.
2. JIT-generated code to host execution environment.
3. One guest VM to another guest VM.
4. Guest VM to supervisor/runtime.
5. Runtime to host operating system and hardware.

## Adversary assumptions

A guest may provide malformed or adversarial instruction streams, invalid addresses, excessive resource requests, and unexpected operation sequences. A guest may attempt to access another guest's resources through any exposed interface.

## Out of scope for the initial prototype

- A malicious or compromised host OS, hypervisor, firmware, or physical hardware.
- Physical attacks and invasive hardware analysis.
- Side-channel resistance.
- Availability against a privileged host administrator.
- Production-grade secure boot, attestation, or encrypted RAM.

## Required future evidence

- Decoder and verifier fuzzing.
- Property tests for capability monotonicity and revocation.
- Cross-VM non-interference tests.
- JIT differential tests against a reference interpreter.
- Memory-safety and concurrency review.
- Independent security assessment before any hardened-security claim.
