//! AEGIS-V64 instruction format and decoder.
//!
//! The current encoding is a prototype contract, not a stable ABI.

/// One fixed-width AEGIS-V64 instruction word.
pub type InstructionWord = u32;

/// Number of general-purpose virtual registers.
pub const GENERAL_REGISTER_COUNT: u8 = 32;

/// Prototype opcode assignments. Values are reserved once published in a stable ISA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0x00,
    Add = 0x01,
    Sub = 0x02,
    Mul = 0x03,
    DivU = 0x04,
    Load64 = 0x10,
    Store64 = 0x11,
    Halt = 0xFF,
}

impl TryFrom<u8> for Opcode {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Nop),
            0x01 => Ok(Self::Add),
            0x02 => Ok(Self::Sub),
            0x03 => Ok(Self::Mul),
            0x04 => Ok(Self::DivU),
            0x10 => Ok(Self::Load64),
            0x11 => Ok(Self::Store64),
            0xFF => Ok(Self::Halt),
            other => Err(DecodeError::UnknownOpcode(other)),
        }
    }
}

/// Decoded fields of a 32-bit instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub opcode: Opcode,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    /// Signed 9-bit immediate, sign-extended to i16.
    pub immediate: i16,
}

/// Errors returned when decoding an instruction word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnknownOpcode(u8),
    RegisterOutOfRange,
}

/// Decode the prototype 32-bit instruction format.
///
/// Layout: opcode[31:24], rd[23:19], rs1[18:14], rs2[13:9], imm[8:0].
pub fn decode(word: InstructionWord) -> Result<DecodedInstruction, DecodeError> {
    let opcode = Opcode::try_from((word >> 24) as u8)?;
    let rd = ((word >> 19) & 0x1f) as u8;
    let rs1 = ((word >> 14) & 0x1f) as u8;
    let rs2 = ((word >> 9) & 0x1f) as u8;
    let raw_imm = (word & 0x1ff) as u16;
    let immediate = if raw_imm & 0x100 != 0 {
        (raw_imm | 0xfe00) as i16
    } else {
        raw_imm as i16
    };

    if rd >= GENERAL_REGISTER_COUNT || rs1 >= GENERAL_REGISTER_COUNT || rs2 >= GENERAL_REGISTER_COUNT {
        return Err(DecodeError::RegisterOutOfRange);
    }

    Ok(DecodedInstruction { opcode, rd, rs1, rs2, immediate })
}

/// Encode fields into the prototype instruction word.
pub fn encode(opcode: Opcode, rd: u8, rs1: u8, rs2: u8, immediate: i16) -> Option<InstructionWord> {
    if rd >= 32 || rs1 >= 32 || rs2 >= 32 || !(-256..=255).contains(&immediate) {
        return None;
    }
    let imm = (immediate as u16) & 0x01ff;
    Some(((opcode as u32) << 24)
        | ((rd as u32) << 19)
        | ((rs1 as u32) << 14)
        | ((rs2 as u32) << 9)
        | imm as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_instruction() {
        let word = encode(Opcode::Add, 3, 4, 5, -7).expect("valid fields");
        let decoded = decode(word).expect("valid instruction");
        assert_eq!(decoded.opcode, Opcode::Add);
        assert_eq!((decoded.rd, decoded.rs1, decoded.rs2), (3, 4, 5));
        assert_eq!(decoded.immediate, -7);
    }

    #[test]
    fn rejects_unknown_opcode() {
        assert_eq!(decode(0x7f00_0000), Err(DecodeError::UnknownOpcode(0x7f)));
    }

    #[test]
    fn rejects_out_of_range_register_on_encode() {
        assert!(encode(Opcode::Add, 32, 0, 0, 0).is_none());
    }
}
