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
    // gets immediate word then increments pc by 2
    fn get_immediate_word(&mut self) -> u16 {
        let value = self.bus.read_word(self.pc.wrapping_add(1));
        self.pc = self.pc.wrapping_add(2);
        value
    }
    fn read_arithmetic_byte_target(&mut self, target: ArithmeticByteTarget) -> u8 {
        let value = match target {
            ArithmeticByteTarget::B => self.registers.b,
            ArithmeticByteTarget::C => self.registers.c,
            ArithmeticByteTarget::D => self.registers.d,
            ArithmeticByteTarget::E => self.registers.e,
            ArithmeticByteTarget::H => self.registers.h,
            ArithmeticByteTarget::L => self.registers.l,
            ArithmeticByteTarget::HL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticByteTarget::A => self.registers.a,
            ArithmeticByteTarget::N8 => self.get_immediate_byte(),
        };
        value
    }
    fn read_arithmetic_word_target(&mut self, target: ArithmeticWordTarget) -> u16 {
        let value = match target {
            ArithmeticWordTarget::BC => self.registers.get_bc(),
            ArithmeticWordTarget::DE => self.registers.get_de(),
            ArithmeticWordTarget::HL => self.registers.get_hl(),
            ArithmeticWordTarget::SP => self.sp,
        };
        value
    }
    fn write_arithmetic_word_target(&mut self, target: ArithmeticWordTarget, value: u16) {
        match target {
            ArithmeticWordTarget::BC => self.registers.set_bc(value),
            ArithmeticWordTarget::DE => self.registers.set_de(value),
            ArithmeticWordTarget::HL => self.registers.set_hl(value),
            ArithmeticWordTarget::SP => self.sp = value,
        }
    }
    fn read_prefixed_target(&mut self, target: PrefixedTarget) -> u8 {
        let r = match target {
            PrefixedTarget::B => self.registers.b,
            PrefixedTarget::C => self.registers.c,
            PrefixedTarget::D => self.registers.d,
            PrefixedTarget::E => self.registers.e,
            PrefixedTarget::H => self.registers.h,
            PrefixedTarget::L => self.registers.l,
            PrefixedTarget::HL => self.bus.read_byte(self.registers.get_hl()),
            PrefixedTarget::A => self.registers.a,
        };
        r
    }
    fn write_prefixed_target(&mut self, target: PrefixedTarget, new_r: u8) {
        match target {
            PrefixedTarget::B => self.registers.b = new_r,
            PrefixedTarget::C => self.registers.c = new_r,
            PrefixedTarget::D => self.registers.d = new_r,
            PrefixedTarget::E => self.registers.e = new_r,
            PrefixedTarget::H => self.registers.h = new_r,
            PrefixedTarget::L => self.registers.l = new_r,
            PrefixedTarget::HL => self.bus.write_byte(self.registers.get_hl(), new_r),
            PrefixedTarget::A => self.registers.a = new_r,
        }
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
            // INC and DEC affect same regs as prefixed instructions
            Instruction::INC(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.INC(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.DEC(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(1)
            }
            // Same as INC and DEC but for 16 bit regs and doesn't touch any flags
            Instruction::INC16(target) => {
                let r = self.read_arithmetic_word_target(target);
                let new_r = r.wrapping_add(1);
                self.write_arithmetic_word_target(target, new_r);
                self.pc.wrapping_add(1)
            }
            Instruction::DEC16(target) => {
                let r = self.read_arithmetic_word_target(target);
                let new_r = r.wrapping_sub(1);
                self.write_arithmetic_word_target(target, new_r);
                self.pc.wrapping_add(1)
            }

            Instruction::ADDHL(target) => {
                let value = self.read_arithmetic_word_target(target);
                let _new_value = self.ADDHL(value);
                self.pc.wrapping_add(1)
            }
            Instruction::ADD(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.ADD(value);
                self.pc.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.ADC(value);
                self.pc.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.SUB(value);
                self.pc.wrapping_add(1)
            }
            Instruction::SBC(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.SBC(value);
                self.pc.wrapping_add(1)
            }
            Instruction::AND(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.AND(value);
                self.pc.wrapping_add(1)
            }
            Instruction::OR(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.OR(value);
                self.pc.wrapping_add(1)
            }
            Instruction::XOR(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.XOR(value);
                self.pc.wrapping_add(1)
            }
            Instruction::CP(target) => {
                let value = self.read_arithmetic_byte_target(target);
                let _new_value = self.CP(value);
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
            // TODO: support for more instructions

            // PREFIXED INSTRUCTIONS
            Instruction::RLC(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.RLC(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::RRC(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.RRC(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::RL(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.RL(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::RR(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.RR(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::SLA(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.SLA(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::SRA(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.SRA(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::SWAP(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.SWAP(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::SRL(target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.SRL(r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::BIT(bit, target) => {
                let r = self.read_prefixed_target(target);
                self.BIT(bit, r);
                self.pc.wrapping_add(2)
            }
            Instruction::RES(bit, target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.RES(bit, r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
            Instruction::SET(bit, target) => {
                let r = self.read_prefixed_target(target);
                let new_r = self.SET(bit, r);
                self.write_prefixed_target(target, new_r);
                self.pc.wrapping_add(2)
            }
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
    // return u16 at current stack pointer (to be stored in 16 bit reg) and increment it
    fn POP(&mut self) -> u16 {
        let result = self.bus.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        result
    }
    // decrement stack pointer and push value from 16 bit reg
    fn PUSH(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.bus.write_word(self.sp, value);
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
    // rotate A right without carry (carry set to old lsb)
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
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        // set new carry to msb of A
        self.registers.f.carry = old_a & 0x80 != 0;
        self.registers.a = new_value;
    }
    // rotate A register right using carry flag
    fn RRA(&mut self) {
        let old_a = self.registers.a;
        let carry_in = if self.registers.f.carry { 0x80 } else { 0 };
        // A shifts right then carry_in becomes msb of A
        let new_value = (old_a >> 1) | carry_in;
        // don't touch zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        // set new carry to lsb of A
        self.registers.f.carry = old_a & 0x1 != 0;
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

    // PREFIXED INSTRUCTIONS
    // rotate r left without carry (carry set to old msb)
    fn RLC(&mut self, r: u8) -> u8 {
        let old_msb = if r & 0x80 != 0 { 0x01 } else { 0 };
        let new_r = (r << 1) | old_msb;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = old_msb != 0;
        new_r
    }
    // rotate r right without carry (carry set to old lsb)
    fn RRC(&mut self, r: u8) -> u8 {
        let old_lsb = if r & 0x01 != 0 { 0x80 } else { 0 };
        let new_r = (r >> 1) | old_lsb;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = old_lsb != 0;
        new_r
    }
    // rotate r left through carry flag
    fn RL(&mut self, r: u8) -> u8 {
        let carry_in = if self.registers.f.carry { 0x01 } else { 0 };
        let new_r = (r << 1) | carry_in;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = r & 0x80 != 0;
        new_r
    }
    // rotate r right through carry flag
    fn RR(&mut self, r: u8) -> u8 {
        let carry_in = if self.registers.f.carry { 0x80 } else { 0 };
        let new_r = (r >> 1) | carry_in;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = r & 0x01 != 0;
        new_r
    }
    // make bit 7 carry and shift left
    fn SLA(&mut self, r: u8) -> u8 {
        let new_r = r << 1;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = r & 0x80 != 0;
        new_r
    }
    // make bit 0 carry and shift right while preserving bit 7
    fn SRA(&mut self, r: u8) -> u8 {
        let old_msb = r & 0x80;
        let new_r = (r >> 1) | old_msb;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = r & 0x01 != 0;
        new_r
    }
    // swap upper and lower nibble of r
    fn SWAP(&mut self, r: u8) -> u8 {
        let lower_nibble = (r & 0x0F) << 4;
        let upper_nibble = (r & 0xF0) >> 4;
        let new_r = lower_nibble | upper_nibble;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_r
    }
    // make bit 0 carry and shift right
    fn SRL(&mut self, r: u8) -> u8 {
        let new_r = r >> 1;
        self.registers.f.zero = new_r == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = r & 0x01 != 0;
        new_r
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