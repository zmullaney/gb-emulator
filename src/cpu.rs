use crate::registers::Registers;
use crate::instructions::*;

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    // pass in address to first byte of u16
    fn read_word(&mut self, address: u16) -> u16 {
        let least_significant_byte = self.read_byte(address) as u16;
        let most_significant_byte = self.read_byte(address.wrapping_add(1)) as u16;
        (most_significant_byte << 8) | least_significant_byte
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    fn write_word(&mut self, address: u16, value: u16) {
        let least_significant_byte = (value & 0xFF) as u8;
        let most_significant_byte = ((value & 0xFF00) >> 8) as u8;
        self.write_byte(address, least_significant_byte);
        self.write_byte(address.wrapping_add(1), most_significant_byte);
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte( self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed { 
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        let next_pc = 
            if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        }   else {
            let description = format!("0x{}{:x}", if prefixed { "CB" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
        self.pc = next_pc;
    }
    // increments pc and returns byte at new pc
    fn get_immediate_byte(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.bus.read_byte(self.pc)
    }
    fn get_immediate_word(&mut self) -> u16 {
        let value = self.bus.read_word(self.pc.wrapping_add(1));
        self.pc = self.pc.wrapping_add(2);
        value
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.JP(jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.JR(jump_condition)
            }
            Instruction::JPHL() => {
                // jump straight to address in HL
                self.registers.get_hl()
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = 
                        match source {
                            LoadSource::A => self.registers.a,
                            LoadSource::B => self.registers.b,
                            LoadSource::C => self.registers.c,
                            LoadSource::D => self.registers.d,
                            LoadSource::E => self.registers.e,
                            LoadSource::H => self.registers.h,
                            LoadSource::L => self.registers.l,
                            LoadSource::BC => self.bus.read_byte(self.registers.get_bc()),
                            LoadSource::DE => self.bus.read_byte(self.registers.get_de()),
                            LoadSource::HL => self.bus.read_byte(self.registers.get_hl()),
                            LoadSource::N8 => self.get_immediate_byte(),
                            _ => panic!("Invalid LD LoadType::Byte source"),
                        };
                        match target {
                            LoadTarget::A => self.registers.a = source_value,
                            LoadTarget::B => self.registers.b = source_value,
                            LoadTarget::C => self.registers.c = source_value,
                            LoadTarget::D => self.registers.d = source_value,
                            LoadTarget::E => self.registers.e = source_value,
                            LoadTarget::H => self.registers.h = source_value,
                            LoadTarget::L => self.registers.l = source_value,
                            LoadTarget::BC => self.bus.write_byte(self.registers.get_bc(), source_value),
                            LoadTarget::DE => self.bus.write_byte(self.registers.get_de(), source_value),
                            LoadTarget::HL => self.bus.write_byte(self.registers.get_hl(), source_value),
                            _ => panic!("Invalid LD LoadType::Byte target"),
                        }
                        self.pc.wrapping_add(1)
                    }
                    LoadType::Word(target, source) => {
                        let source_value = 
                        match source {
                            LoadSource::N16 => self.get_immediate_word(),
                            LoadSource::SP => self.sp,
                            _ => panic!("Invalid LD LoadType::Word source"),
                        };
                        match target {
                            LoadTarget::BC => self.registers.set_bc(source_value),
                            LoadTarget::DE => self.registers.set_de(source_value),
                            LoadTarget::HL => self.registers.set_hl(source_value),
                            LoadTarget::SP => self.sp = source_value,
                            LoadTarget::A16 => {
                                let address = self.get_immediate_word();
                                self.bus.write_word(address, source_value)
                            }
                            _ => panic!("Invalid LD LoadType::Word target"),
                        }
                        // increments from immediate values / addresses should be handled in get_immediate_word
                        self.pc.wrapping_add(1)
                    }
                    // _ => panic!("Invalid LD LoadType"),
                }
            }
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
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.ADD(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for ADD"),
                }
                self.pc.wrapping_add(1)
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
                self.pc.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.ADC(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.ADC(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for ADC"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.SUB(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.SUB(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for SUB"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::SBC(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.SBC(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.SBC(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for SBC"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::AND(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.AND(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.AND(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for AND"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::OR(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.OR(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.OR(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for OR"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::XOR(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.XOR(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.XOR(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for XOR"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::CP(target) => {
                match target {
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
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let _new_value = self.CP(value);
                    }
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let _new_value = self.CP(value);
                    }
                    ArithmeticTarget::N8 => {
                        let value = self.get_immediate_byte();
                        let _new_value = self.CP(value);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for CP"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::INC(target) => {
                match target {
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
                    ArithmeticTarget::A => {
                        self.registers.a = self.INC(self.registers.a);
                    }
                    // TODO: HL as pointer operand (no immediate value support)
                    _ => panic!("Incompatible ArithmeticTarget for INC"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                match target {
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
                    ArithmeticTarget::A => {
                        self.registers.a = self.DEC(self.registers.a);
                    }
                    // TODO: HL as pointer operand (no immediate value support)
                    _ => panic!("Incompatible ArithmeticTarget for DEC"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::CCF() => {
                self.CCF();
                self.pc.wrapping_add(1)
            }
            Instruction::SCF() => {
                self.SCF();
                self.pc.wrapping_add(1)
            }
            Instruction::RRA() => {
                self.RRA();
                self.pc.wrapping_add(1)
            }
            Instruction::RLA() => {
                self.RLA();
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA() => {
                self.RRCA();
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA() => {
                self.RLCA();
                self.pc.wrapping_add(1)
            }
            Instruction::CPL() => {
                self.CPL();
                self.pc.wrapping_add(1)
            }
            // TODO: support for more instructions
            // _ => panic!("Unknown instruction (exited execute)"),
        }
    }

    // jump to 16 bit address stored after instruction
    fn JP(&mut self, should_jump: bool) -> u16 {
        if should_jump { self.bus.read_word(self.pc.wrapping_add(1)) }
        else { self.pc.wrapping_add(3) }
    }
    // jump based on i8 offset stored after instruction
    fn JR(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let offset = self.bus.read_byte(self.pc.wrapping_add(1)) as i8;
            // compiler demands i16 for wrapping_add_signed here
            self.pc.wrapping_add_signed(offset as i16)
        }
        else { self.pc.wrapping_add(2) }
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
        // A shifts right then carry_in becomes msb of A
        let new_value = (old_a >> 1) | carry_in;
        // set new carry to lsb of A
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
        // A shifts left then carry_in becomes lsb of A
        let new_value = (old_a << 1) | carry_in;
        // set new carry to msb of A
        self.registers.f.carry = old_a & 0x80 != 0;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    // rotate A reg right without carry (carry set to old lsb)
    fn RRCA(&mut self) {
        let old_a = self.registers.a;
        let old_lsb = if old_a & 0x01 != 0 { 0x80 } else { 0 };
        let new_value = (old_a >> 1) | old_lsb;
        self.registers.f.carry = old_lsb != 0;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.a = new_value;
    }
    // rotate A left without carry (carry set to old msb)
    fn RLCA(&mut self) {
        let old_a = self.registers.a;
        let old_msb = if old_a & 0x80 != 0 { 0x01 } else { 0 };
        let new_value = (old_a << 1) | old_msb;
        self.registers.f.carry = old_msb != 0;
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