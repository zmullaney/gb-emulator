use crate::registers::Registers;
use crate::instructions::{Instruction, ArithmeticTarget};

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found for: 0x{:x}", instruction_byte);
        };
        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.ADD(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.ADD(value);
                    }
                    // TODO: add immediate value support
                    _ => panic!("Incompatible ArithmeticTarget for ADD"),
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget::BC => {
                        let value = self.registers.get_bc();
                        let _new_value = self.ADDHL(value);
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let _new_value = self.ADDHL(value);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let _new_value = self.ADDHL(value);
                    }
                    // TODO: ADDHL for SP (Stack Pointer)
                    _ => panic!("Incompatible ArithmeticTarget for ADDHL"),
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.ADC(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for ADC"),
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.SUB(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for SUB"),
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.SBC(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for SBC"),
                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.AND(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for AND"),
                }
            }
            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.OR(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for OR"),
                }
            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let _new_value = self.XOR(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for XOR"),
                }
            }
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.CP(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.CP(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.CP(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.CP(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.CP(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.CP(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.CP(value);
                    }
                    // TODO: address in HL and imm value support
                    _ => panic!("Incompatible ArithmeticTarget for CP"),
                }
            }
            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.INC(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.INC(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.INC(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.INC(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.INC(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.INC(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.INC(self.registers.l);
                    }
                    // TODO: HL as pointer operand (no immediate value support)
                    _ => panic!("Incompatible ArithmeticTarget for INC"),
                }
            }
            Instruction::DEC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.DEC(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.DEC(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.DEC(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.DEC(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.DEC(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.DEC(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.DEC(self.registers.l);
                    }
                    // TODO: HL as pointer operand (no immediate value support)
                    _ => panic!("Incompatible ArithmeticTarget for DEC"),
                }
            }
            Instruction::CCF() => self.CCF(),
            Instruction::SCF() => self.SCF(),
            Instruction::RRA() => self.RRA(),
            Instruction::RLA() => self.RLA(),
            Instruction::RRCA() => self.RRCA(),
            Instruction::RLCA() => self.RLCA(),
            Instruction::CPL() => self.CPL(),
            // TODO: support for more instructions
            _ => panic!("Unknown instruction (exited from cpu.rs)"),
        }
        // return next_pc
        self.pc.wrapping_add(1)
    }

    fn ADD(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // set half carry based on if lower nibbles added overflows to upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;
        self.registers.a = new_value;
        new_value
    }
    // like ADD except the target is added to the HL reg and it's 16 bit based
    fn ADDHL(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (new_value, did_overflow) = hl.overflowing_add(value);
        // self.registers.f.zero = new_value == 0;  <- don't touch in ADDHL
        self.registers.f.subtract = false;
        // check for uppermost nibble carry (bit 11 -> 12)
        self.registers.f.half_carry = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.f.carry = did_overflow;
        self.registers.set_hl(new_value);
        new_value
    }
    // like ADD except the value of carry flag is also added to the number (as 0b1)
    // checks for carry from both additions
    fn ADC(&mut self, value: u8) -> u8 {
        let carry_in = if self.registers.f.carry { 1 } else { 0 };
        let (intermediate, carry1) = self.registers.a.overflowing_add(value);
        let (new_value, carry2) = intermediate.overflowing_add(carry_in);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + carry_in > 0xF;
        self.registers.f.carry = carry1 || carry2;
        self.registers.a = new_value;
        new_value
    }
    // subtract value from reg A and store into A
    fn SUB(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // "overflow" is effectively checking underflow in this case
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;
        self.registers.a = new_value;
        new_value
    }
    // like SUB but also with carry (ADC for SUB)
    fn SBC(&mut self, value: u8) -> u8 {
        let carry_in = if self.registers.f.carry { 1 } else { 0 };
        let (intermediate, carry1) = self.registers.a.overflowing_sub(value);
        let (new_value, carry2) = intermediate.overflowing_sub(carry_in);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + carry_in;
        self.registers.f.carry = carry1 || carry2;
        self.registers.a = new_value;
        new_value
    }
    // bitwise AND with A reg
    fn AND(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // arbitrarily set half_carry to true
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        self.registers.a = new_value;
        new_value
    }
    fn OR(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        self.registers.a = new_value;
        new_value
    }
    fn XOR(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        self.registers.a = new_value;
        new_value
    }
    // sets flags like SUB but doesn't store in A
    fn CP(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;
    }
    // increment input register
    #[allow(non_snake_case)]
    fn INC(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) == 0xF;
        // don't touch carry flag in INC or DEC
        new_value
    }
    // decrement input register
    fn DEC(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) == 0;
        // don't touch carry flag in INC or DEC
        new_value
    }
    // toggle carry bit
    fn CCF(&mut self) {
        self.registers.f.carry = !self.registers.f.carry;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    // set carry bit to true
    fn SCF(&mut self) {
        self.registers.f.carry = true;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    // rotate A register right using carry flag
    fn RRA(&mut self) {
        let old_a = self.registers.a;
        let carry_in = if self.registers.f.carry { 0x80 } else { 0 };
        // A shifts right then carry_in becomes MSB of A
        let new_value = (old_a >> 1) | carry_in;
        // set new carry to LSB of A
        self.registers.f.carry = old_a & 0x1 != 0;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    // rotate A register left using carry flag
    fn RLA(&mut self) {
        let old_a = self.registers.a;
        let carry_in = if self.registers.f.carry { 0x01 } else { 0 };
        // A shifts left then carry_in becomes LSB of A
        let new_value = (old_a << 1) | carry_in;
        // set new carry to MSB of A
        self.registers.f.carry = old_a & 0x80 != 0;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    // rotate A reg right without carry (carry set to old LSB)
    fn RRCA(&mut self) {
        let old_a = self.registers.a;
        let old_LSB = if old_a & 0x01 != 0 { 0x80 } else { 0 };
        let new_value = (old_a >> 1) | old_LSB;
        self.registers.f.carry = old_LSB != 0;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    // rotate A left without carry (carry set to old MSB)
    fn RLCA(&mut self) {
        let old_a = self.registers.a;
        let old_MSB = if old_a & 0x80 != 0 { 0x01 } else { 0 };
        let new_value = (old_a << 1) | old_MSB;
        self.registers.f.carry = old_MSB != 0;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    // toggle every bit in A
    fn CPL(&mut self) {
        let new_a = self.registers.a ^ 0xFF;
        // don't touch zero or carry flags
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        self.registers.a = new_a;
    }
}