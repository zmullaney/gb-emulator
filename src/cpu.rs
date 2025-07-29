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
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::BC => self.bus.read_byte(self.registers.get_bc()),
                            LoadByteSource::DE => self.bus.read_byte(self.registers.get_de()),
                            LoadByteSource::HL => self.bus.read_byte(self.registers.get_hl()),
                            LoadByteSource::N8 => self.get_immediate_byte(),
                            // _ => panic!("Invalid LD LoadType::Byte source"),
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::BC => self.bus.write_byte(self.registers.get_bc(), source_value),
                            LoadByteTarget::DE => self.bus.write_byte(self.registers.get_de(), source_value),
                            LoadByteTarget::HL => self.bus.write_byte(self.registers.get_hl(), source_value),
                            // _ => panic!("Invalid LD LoadType::Byte target"),
                        }
                        self.pc.wrapping_add(1)
                    }
                    LoadType::Word(target, source) => {
                        let source_value = 
                        match source {
                            LoadWordSource::N16 => self.get_immediate_word(),
                            LoadWordSource::SP => self.sp,
                            // _ => panic!("Invalid LD LoadType::Word source"),
                        };
                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(source_value),
                            LoadWordTarget::DE => self.registers.set_de(source_value),
                            LoadWordTarget::HL => self.registers.set_hl(source_value),
                            LoadWordTarget::SP => self.sp = source_value,
                            LoadWordTarget::A16 => {
                                let address = self.get_immediate_word();
                                self.bus.write_word(address, source_value)
                            }
                            // _ => panic!("Invalid LD LoadType::Word target"),
                        }
                        // increments from immediate values / addresses should be handled in get_immediate_word
                        self.pc.wrapping_add(1)
                    }
                    LoadType::AddressIncDec(target, source, mode) => {
                        let hl = self.registers.get_hl();
                        match (target, source) {
                            (LoadIncDecTarget::A, LoadIncDecSource::HL) => self.registers.a = self.bus.read_byte(hl),
                            (LoadIncDecTarget::HL, LoadIncDecSource::A) => self.bus.write_byte(hl, self.registers.a),
                            _ => panic!("Invalid AddressIncDec inputs"),
                        };
                        let new_hl = match mode {
                            AddressMode::Inc => hl.wrapping_add(1),
                            AddressMode::Dec => hl.wrapping_sub(1),
                        };
                        self.registers.set_hl(new_hl);
                        self.pc.wrapping_add(1)
                    }
                    // _ => panic!("Invalid LD LoadType"),
                }
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };
                self.PUSH(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let value = self.POP();
                match target {
                    StackTarget::BC => self.registers.set_bc(value),
                    StackTarget::DE => self.registers.set_de(value),
                    StackTarget::HL => self.registers.set_hl(value),
                    StackTarget::AF => self.registers.set_af(value),
                }
                self.pc.wrapping_add(1)
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
                    ArithmeticTarget::SP => {
                        let value = self.sp;
                        let _new_value = self.ADDHL(value);
                    }
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
                    ArithmeticTarget::HL => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.INC(value);
                        self.bus.write_byte(hl, new_value);
                    }
                    ArithmeticTarget::A => {
                        self.registers.a = self.INC(self.registers.a);
                    }
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
                    ArithmeticTarget::HL => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let new_value = self.DEC(value);
                        self.bus.write_byte(hl, new_value);
                    }
                    ArithmeticTarget::A => {
                        self.registers.a = self.DEC(self.registers.a);
                    }
                    _ => panic!("Incompatible ArithmeticTarget for DEC"),
                }
                self.pc.wrapping_add(1)
            }
            // Same as INC and DEC but for 16 bit regs and doesn't touch any flags
            Instruction::INC16(target) => {
                match target {
                    ArithmeticTarget::BC => self.registers.set_bc(self.registers.get_bc().wrapping_add(1)),
                    ArithmeticTarget::DE => self.registers.set_de(self.registers.get_de().wrapping_add(1)),
                    ArithmeticTarget::HL => self.registers.set_hl(self.registers.get_hl().wrapping_add(1)),
                    ArithmeticTarget::SP => self.sp = self.sp.wrapping_add(1),
                    _ => panic!("Incompatible ArithmeticTarget for INC16"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::DEC16(target) => {
                match target {
                    ArithmeticTarget::BC => self.registers.set_bc(self.registers.get_bc().wrapping_sub(1)),
                    ArithmeticTarget::DE => self.registers.set_de(self.registers.get_de().wrapping_sub(1)),
                    ArithmeticTarget::HL => self.registers.set_hl(self.registers.get_hl().wrapping_sub(1)),
                    ArithmeticTarget::SP => self.sp = self.sp.wrapping_sub(1),
                    _ => panic!("Incompatible ArithmeticTarget for DEC16"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA() => {
                self.RLCA();
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA() => {
                self.RRCA();
                self.pc.wrapping_add(1)
            }
            Instruction::RLA() => {
                self.RLA();
                self.pc.wrapping_add(1)
            }
            Instruction::RRA() => {
                self.RRA();
                self.pc.wrapping_add(1)
            }
            Instruction::CPL() => {
                self.CPL();
                self.pc.wrapping_add(1)
            }
            Instruction::SCF() => {
                self.SCF();
                self.pc.wrapping_add(1)
            }
            Instruction::CCF() => {
                self.CCF();
                self.pc.wrapping_add(1)
            }

            // PREFIXED INSTRUCTIONS
            Instruction::BIT(bit, target) => {
                match target {
                    ArithmeticTarget::B => self.BIT(bit, self.registers.b),
                    ArithmeticTarget::C => self.BIT(bit, self.registers.c),
                    ArithmeticTarget::D => self.BIT(bit, self.registers.d),
                    ArithmeticTarget::E => self.BIT(bit, self.registers.e),
                    ArithmeticTarget::H => self.BIT(bit, self.registers.h),
                    ArithmeticTarget::L => self.BIT(bit, self.registers.l),
                    ArithmeticTarget::HL => {
                        let hl = self.registers.get_hl();
                        let r = self.bus.read_byte(hl);
                        self.BIT(bit, r);
                    }
                    ArithmeticTarget::A => self.BIT(bit, self.registers.a),
                    _ => panic!("Incompatible ArithmeticTarget for BIT"),
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RES(bit, target) => {
                match target {
                    ArithmeticTarget::B => {
                        let r = self.registers.b;
                        let new_r = self.RES(bit, r);
                        self.registers.b = new_r;
                    }
                    ArithmeticTarget::C => {
                        let r = self.registers.c;
                        let new_r = self.RES(bit, r);
                        self.registers.c = new_r;
                    }
                    ArithmeticTarget::D => {
                        let r = self.registers.d;
                        let new_r = self.RES(bit, r);
                        self.registers.d = new_r;
                    }
                    ArithmeticTarget::E => {
                        let r = self.registers.e;
                        let new_r = self.RES(bit, r);
                        self.registers.e = new_r;
                    }
                    ArithmeticTarget::H => {
                        let r = self.registers.h;
                        let new_r = self.RES(bit, r);
                        self.registers.h = new_r;
                    }
                    ArithmeticTarget::L => {
                        let r = self.registers.l;
                        let new_r = self.RES(bit, r);
                        self.registers.l = new_r;
                    }
                    ArithmeticTarget::HL => {
                        let hl = self.registers.get_hl();
                        let r = self.bus.read_byte(hl);
                        let new_r = self.RES(bit, r);
                        self.bus.write_byte(hl, new_r);
                    }
                    ArithmeticTarget::A => {
                        let r = self.registers.a;
                        let new_r = self.RES(bit, r);
                        self.registers.a = new_r;
                    }
                    _ => panic!("Incompatible ArithmeticTarget for RES"),
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SET(bit, target) => {
                match target {
                    ArithmeticTarget::B => {
                        let r = self.registers.b;
                        let new_r = self.SET(bit, r);
                        self.registers.b = new_r;
                    }
                    ArithmeticTarget::C => {
                        let r = self.registers.c;
                        let new_r = self.SET(bit, r);
                        self.registers.c = new_r;
                    }
                    ArithmeticTarget::D => {
                        let r = self.registers.d;
                        let new_r = self.SET(bit, r);
                        self.registers.d = new_r;
                    }
                    ArithmeticTarget::E => {
                        let r = self.registers.e;
                        let new_r = self.SET(bit, r);
                        self.registers.e = new_r;
                    }
                    ArithmeticTarget::H => {
                        let r = self.registers.h;
                        let new_r = self.SET(bit, r);
                        self.registers.h = new_r;
                    }
                    ArithmeticTarget::L => {
                        let r = self.registers.l;
                        let new_r = self.SET(bit, r);
                        self.registers.l = new_r;
                    }
                    ArithmeticTarget::HL => {
                        let hl = self.registers.get_hl();
                        let r = self.bus.read_byte(hl);
                        let new_r = self.SET(bit, r);
                        self.bus.write_byte(hl, new_r);
                    }
                    ArithmeticTarget::A => {
                        let r = self.registers.a;
                        let new_r = self.SET(bit, r);
                        self.registers.a = new_r;
                    }
                    _ => panic!("Incompatible ArithmeticTarget for SET"),
                }
                self.pc.wrapping_add(2)
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
    // decrement stack pointer and push value from 16 bit reg
    fn PUSH(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.bus.write_word(self.sp, value);
    }
    // return u16 at current stack pointer (to be stored in 16 bit reg) and increment it
    fn POP(&mut self) -> u16 {
        let result = self.bus.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        result
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
    // toggle every bit in A
    fn CPL(&mut self) {
        let new_a = self.registers.a ^ 0xFF;
        // don't touch zero or carry flags
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        self.registers.a = new_a;
    }
    // set carry bit to true
    fn SCF(&mut self) {
        self.registers.f.carry = true;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    // toggle carry bit
    fn CCF(&mut self) {
        self.registers.f.carry = !self.registers.f.carry;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    // test whether bit in question is set
    fn BIT(&mut self, bit: u8, r: u8) {
        let bit_set = r & (1 << bit) != 0;
        self.registers.f.zero = !bit_set;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        // don't touch carry flag
    }
    // reset bit in question to 0
    fn RES(&mut self, bit: u8, r: u8) -> u8 {
        let new_r = r & !(1 << bit);
        // don't touch any flags
        new_r
    }
    // set bit in question to 1
    fn SET(&mut self, bit: u8, r: u8) -> u8 {
        let new_r = r | (1 << bit);
        // don't touch any flags
        new_r
    }
}