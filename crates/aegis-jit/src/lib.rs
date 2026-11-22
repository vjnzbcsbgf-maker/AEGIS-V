//! JIT pipeline boundary.
//!
//! Native code generation is deliberately not implemented in this initial commit.
//! A reference interpreter and verifier must define and validate semantics before
//! executable AMD64 code is emitted.

use aegis_isa::{decode, DecodeError, DecodedInstruction, InstructionWord};

/// Errors produced by the current translation front-end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitError {
    Decode(DecodeError),
    BackendNotImplemented,
}

/// A decoded block ready for verification and later translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedBlock {
    pub instructions: Vec<DecodedInstruction>,
}

/// Decode a sequence of instruction words. No native code is emitted.
pub fn decode_block(words: &[InstructionWord]) -> Result<DecodedBlock, JitError> {
    let instructions = words.iter().copied().map(|word| decode(word).map_err(JitError::Decode))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(DecodedBlock { instructions })
}

/// Placeholder for the future AMD64 code-generation backend.
pub fn compile_amd64(_block: &DecodedBlock) -> Result<Vec<u8>, JitError> {
    Err(JitError::BackendNotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aegis_isa::{encode, Opcode};

    #[test]
    fn front_end_decodes_block() {
        let word = encode(Opcode::Nop, 0, 0, 0, 0).unwrap();
        let block = decode_block(&[word]).unwrap();
        assert_eq!(block.instructions.len(), 1);
    }

    #[test]
    fn backend_is_explicitly_unimplemented() {
        let block = DecodedBlock { instructions: vec![] };
        assert_eq!(compile_amd64(&block), Err(JitError::BackendNotImplemented));
    }
}
