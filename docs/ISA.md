# AEGIS-V64 ISA — draft 0.1

Status: provisional design. Encoding and opcode assignments may change before a stable specification.

## Machine model

- 64-bit virtual machine word and address model.
- Little-endian guest memory.
- 32 general-purpose registers, each 64 bits wide; register zero is not yet hardwired.
- Fixed-width 32-bit instruction words in the first encoding revision.
- Host pointers are never part of the guest-visible ISA.
- Capability handles are typed runtime objects, not integer addresses.

## Instruction encoding

| Bits | Width | Field |
|---|---:|---|
| 31..24 | 8 | opcode |
| 23..19 | 5 | rd |
| 18..14 | 5 | rs1 |
| 13..9 | 5 | rs2 |
| 8..0 | 9 | signed immediate |

The immediate is sign-extended from 9 bits (range -256 through 255). Fields unused by an opcode must be zero in a future strict verifier; the current decoder only extracts fields.

## Initial opcode assignments

| Opcode | Mnemonic | Initial meaning |
|---:|---|---|
| 0x00 | NOP | No operation |
| 0x01 | ADD | Unsigned 64-bit wrapping addition |
| 0x02 | SUB | Unsigned 64-bit wrapping subtraction |
| 0x03 | MUL | Low 64 bits of multiplication |
| 0x04 | DIVU | Unsigned division; zero divisor traps |
| 0x10 | LOAD64 | Capability-checked 64-bit load (semantics pending) |
| 0x11 | STORE64 | Capability-checked 64-bit store (semantics pending) |
| 0xFF | HALT | Stop the current VM |

These are reserved draft assignments. The current repository implements encoding/decoding only; it does not yet execute these operations.

## Capability model

Capabilities are opaque references to objects with bounded authority. Derivation must be monotonic: child rights and bounds are subsets of the parent authority. Revocation invalidates the issued handle. The draft does not yet specify cross-process representation, persistence, or delegation over IPC.

## Traps and exceptions

The stable ISA must define invalid opcode, invalid register, arithmetic fault, memory fault, capability fault, and explicit halt behavior. Precise exception semantics are not implemented yet.
