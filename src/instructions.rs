pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
pub enum LoadByteTarget {
    A, B, C, D, E, H, L, BC, DE, HL,
}
pub enum LoadByteSource {
    A, B, C, D, E, H, L, BC, DE, HL, N8,
}
pub enum LoadWordTarget {
    BC, DE, HL, SP, A16,
}
pub enum LoadWordSource {
    N16, SP,
}
pub enum LoadIncDecTarget {
    HL, A,
}
pub enum LoadIncDecSource {
    HL, A,
}
pub enum AddressMode {
    Inc, Dec
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
    AddressIncDec(LoadIncDecTarget, LoadIncDecSource, AddressMode),
}
pub enum StackTarget {
    BC, DE, HL, AF,
}
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, BC, DE, HL, SP, N8,
}
pub enum Instruction {
    JP(JumpTest),
    JR(JumpTest),
    JPHL(),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    INC16(ArithmeticTarget),
    DEC16(ArithmeticTarget),
    CCF(),
    SCF(),
    RRA(),
    RLA(),
    RRCA(),
    RLCA(),
    CPL(),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_standard(byte)
        }
    }

    fn from_byte_standard(byte: u8) -> Option<Instruction> {
        match byte {
            // LD Byte, Target B
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::N8))),
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HL))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            // LD Byte, Target C
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::N8))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HL))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),
            // LD Byte, Target D
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::N8))),
            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            // LD Byte, Target E
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::N8))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
            // LD Byte, Target H
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::N8))),
            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            // LD Byte, Target L
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::N8))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),
            // LD Byte, Target [HL]
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::N8))),
            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::L))),
            // 0x76 => HALT
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::A))),
            // LD Byte, Target A
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::N8))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),
            // INC
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),
            0x34 => Some(Instruction::INC(ArithmeticTarget::HL)),
            0x3C => Some(Instruction::INC(ArithmeticTarget::A)),
            // DEC
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x35 => Some(Instruction::DEC(ArithmeticTarget::HL)),
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A)),
            // INC16
            0x03 => Some(Instruction::INC16(ArithmeticTarget::BC)),
            0x13 => Some(Instruction::INC16(ArithmeticTarget::DE)),
            0x23 => Some(Instruction::INC16(ArithmeticTarget::HL)),
            0x33 => Some(Instruction::INC16(ArithmeticTarget::SP)),
            // DEC16
            0x0B => Some(Instruction::DEC16(ArithmeticTarget::BC)),
            0x1B => Some(Instruction::DEC16(ArithmeticTarget::DE)),
            0x2B => Some(Instruction::DEC16(ArithmeticTarget::HL)),
            0x3B => Some(Instruction::DEC16(ArithmeticTarget::SP)),
            // ADDHL
            0x09 => Some(Instruction::ADDHL(ArithmeticTarget::BC)),
            0x19 => Some(Instruction::ADDHL(ArithmeticTarget::DE)),
            0x29 => Some(Instruction::ADDHL(ArithmeticTarget::HL)),
            0x39 => Some(Instruction::ADDHL(ArithmeticTarget::SP)),
            // ADD
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::N8)),
            // ADC
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::N8)),
            // SUB
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::N8)),
            // SBC
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xDE => Some(Instruction::SBC(ArithmeticTarget::N8)),
            // AND
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::N8)),
            // XOR
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xEE => Some(Instruction::XOR(ArithmeticTarget::N8)),
            // OR
            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::N8)),
            // CP
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
            0xFE => Some(Instruction::CP(ArithmeticTarget::N8)),
            // POP
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            // PUSH
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            _ => panic!("Unknown instruction (exited from_byte_standard)"),
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {

            _ => panic!("Unknown instruction (exited from_byte_prefixed)"),
        }
    }
}