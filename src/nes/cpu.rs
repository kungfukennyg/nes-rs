use super::memory;
use super::memory::Memory;
use super::memory::NesMemory;
use std::cell::RefCell;

pub static CARRY_BIT: u8 = 1 << 0;
pub static ZERO_FLAG: u8 = 1 << 1;
pub static INTERRUPT_FLAG: u8 = 1 << 2;
pub static BREAK_FLAG: u8 = 1 << 4;
pub static OVERFLOW_FLAG: u8 = 1 << 6;
pub static NEGATIVE_FLAG: u8 = 1 << 7;

const NMI_ADDR: u16 = 0xfffa;
const RESET_ADDR: u16 = 0xfffc;
const BRK_ADDR: u16 = 0xfffe;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: NesMemory,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            memory: NesMemory::new(),
        }
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.memory.fetch(self.registers.program_counter);
        self.registers.program_counter += 1;
        println!("Loaded opcode: {:x}", opcode);
        let mut cycles = 0;

        match opcode {

            // Storage

            // LDA
            0xa1 | 0xa5 | 0xa9 | 0xad | 0xb1 | 0xb5 | 0xb9 | 0xbd => {
                let result = self.alu_address(opcode);
                self.lda(result.0);
                cycles = result.1;
            }
            // LDX
            0xa2 | 0xa6 | 0xae | 0xb6 | 0xbe => {
                let result = self.rmw_address(opcode);
                self.ldx(result.0);
                cycles = result.1;
            }
            // LDY
            0xa0 | 0xa4 | 0xac | 0xb4 | 0xbc => {
                let result = self.control_address(opcode);
                self.ldy(result.0);
                cycles = result.1;
            }
            // STA
            0x81 | 0x85 | 0x8d | 0x91 | 0x95 | 0x99 | 0x9d => {
                let result = self.alu_address(opcode);
                self.sta(result.0);
                cycles = result.1;
            }
            // STX
            0x86 | 0x8e | 0x96 => {
                let result = self.rmw_address(opcode);
                self.stx(result.0);
                cycles = result.1;
            }
            // STY
            0x84 | 0x8c | 0x94 => {
                let result = self.control_address(opcode);
                self.sty(result.0);
                cycles = result.1;
            }
            // TAX
            0xaa => {
                cycles = 2;
                self.tax();
            }
            // TAY
            0xa8 => {
                cycles = 2;
                self.tay();
            }
            // TXA
            0x8a => {
                cycles = 2;
                self.txa();
            }
            // TYA
            0x98 => {
                cycles = 2;
                self.tya();
            }
            // TSX
            0xba => {
                cycles = 2;
                self.tsx();
            },
            // TXS
            0x9a => {
                cycles = 2;
                self.txs();
            }

            // Stack

            // PHA
            0x48 => {
                cycles = 3;
                self.pha();
            }
            // PHP
            0x08 => {
                cycles = 3;
                self.php();
            }
            // PLA
            0x68 => {
                cycles = 4;
                self.pla();
            }
            // PLP
            0x28 => {
                cycles = 4;
                self.plp();
            }

            // Bitwise

            // AND
            0x21 | 0x25 | 0x29 | 0x2d | 0x31 | 0x35 | 0x39 | 0x3d => {
                let result = self.alu_address(opcode);
                self.and(result.0);
                cycles = result.1;
            }
            // EOR
            0x41 | 0x45 | 0x49 | 0x4d | 0x51 | 0x55 | 0x59 | 0x5d => {
                let result = self.alu_address(opcode);
                self.eor(result.0);
                cycles = result.1;
            }
            // ORA
            0x01 | 0x05 | 0x09 | 0x0d | 0x11 | 0x15 | 0x19 | 0x1d => {
                let result = self.alu_address(opcode);
                self.ora(result.0);
                cycles = result.1;
            }
            // BIT
            0x24 | 0x2c => {
                let result = self.control_address(opcode);
                self.bit(result.0);
                cycles = result.1;
            }
            // ASL
            0x0a | 0x06 | 0x16 | 0x0e | 0x1e => {
                if opcode == 0x0a {
                    self.asl_accumulator();
                    cycles = 2;
                } else {
                    let address;
                    match opcode {
                        0x06 => {
                            address = self.zero_page_address();
                            cycles = 5;
                        }
                        0x16 => {
                            address = self.zero_page_indexed_address(Index::X);
                            cycles = 6;
                        }
                        0x1e => {
                            let result = self.absolute_indexed_address(Index::X);
                            address = result.0;
                            cycles = 7 + result.1;
                        }
                        0x0e => {
                            address = self.absolute_address();
                            cycles = 6;
                        }
                        _ => panic!("Unreachable")
                    }

                    self.asl(address);
                }
            }
            // LSR
            0x4a | 0x46 | 0x56 | 0x4e | 0x5e => {
                if opcode == 0x4a {
                    self.lsr_accumulator();
                    cycles = 2;
                }
                else
                {
                    let address;
                    match opcode {
                        0x46 => {
                            address = self.zero_page_address();
                            cycles = 5;
                        }
                        0x56 => {
                            address = self.zero_page_indexed_address(Index::X);
                            cycles = 6;
                        }
                        0x4e => {
                            address = self.absolute_address();
                            cycles = 6;
                        }
                        0x5e => {
                            let result = self.absolute_indexed_address(Index::X);
                            address = result.0;
                            cycles = 7 + result.1;
                        }
                        _ => panic!("Unreachable")
                    }

                    self.lsr(address);
                }
            }
            // ROL
            0x2a | 0x26 | 0x36 | 0x2e | 0x3e => {
                if opcode == 0x2a {
                    self.rol_accumulator();
                    cycles = 2;
                } else {
                    let address;
                    match opcode {
                        0x26 => {
                            address = self.zero_page_address();
                            cycles = 5;
                        }
                        0x36 => {
                            address = self.zero_page_indexed_address(Index::X);
                            cycles = 6;
                        }
                        0x2e => {
                            address = self.absolute_address();
                            cycles = 6;
                        }
                        0x3e => {
                            let result = self.absolute_indexed_address(Index::X);
                            address = result.0;
                            cycles = 7 + result.1;
                        }
                        _ => panic!("Unreachable")
                    }

                    self.rol(address);
                }
            }
            // ROR
            0x6a | 0x66 | 0x76 | 0x6e | 0x7e => {
                if opcode == 0x6a {
                    self.ror_accumulator();
                    cycles = 2;
                } else {
                    let address;
                    match opcode {
                        0x66 => {
                            address = self.zero_page_address();
                            cycles = 5;
                        }
                        0x76 => {
                            address = self.zero_page_indexed_address(Index::X);
                            cycles = 6;
                        }
                        0x6e => {
                            address = self.absolute_address();
                            cycles = 6;
                        }
                        0x7e => {
                            let result = self.absolute_indexed_address(Index::X);
                            address = result.0;
                            cycles = 7 + result.1;
                        }
                        _ => panic!("Unreachable")
                    }
                    self.ror(address);
                }
            }

            // Math

            // ADC
            0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x75 | 0x79 | 0x7d => {
                let result = self.alu_address(opcode);
                self.adc(result.0);
                cycles = result.1;
            }
            //SBC
            0xe1 | 0xe5 | 0xeb | 0xe9 | 0xed | 0xf1 | 0xf5 | 0xf9 | 0xfd => {
                let result = self.alu_address(opcode);
                self.sbc(result.0);
                cycles = result.1;
            }
            // DEC
            0xc6 | 0xd6 | 0xce | 0xde => {
                let address;
                match opcode {
                    0xc6 => {
                        address = self.zero_page_address();
                        cycles = 5;
                    }
                    0xd6 => {
                        address = self.zero_page_indexed_address(Index::X);
                        cycles = 6;
                    }
                    0xce => {
                        address = self.absolute_address();
                        cycles = 6;
                    }
                    0xde => {
                        let result = self.absolute_indexed_address(Index::X);
                        address = result.0;
                        cycles = 7 + result.1;
                    }
                    _ => panic!("Unreachable")
                }

                self.dec(address);
            }
            // DEX
            0xca => {
                self.dex();
                cycles = 2;
            }
            // DEY
            0x88 => {
                self.dey();
                cycles = 2;
            }
            // INC
            0xe6 | 0xf6 | 0xfe | 0xee => {
                let address;
                match opcode {
                    0xe6 => {
                        address = self.zero_page_address();
                        cycles = 6;
                    }
                    0xf6 => {
                        address = self.zero_page_indexed_address(Index::X);
                        cycles = 6;
                    }
                    0xfe => {
                        let result = self.absolute_indexed_address(Index::X);
                        address = result.0;
                        cycles = 7 + result.1;
                    }
                    0xee => {
                        address = self.absolute_address();
                        cycles = 6;
                    }
                    _ => panic!("Unreachable")
                }

                self.inc(address);
            }
            // INX
            0xe8 => {
                self.inx();
                cycles = 2;
            }
            // INY
            0xc8 => {
                self.iny();
                cycles = 2;
            }

            // Registers

            // CLC
            0x18 => {
                self.clc();
                cycles = 2;
            }
            // CLI
            0x58 => {
                self.cli();
                cycles = 2;
            }
            // CLV
            0xb8 => {
                self.clv();
                cycles = 2;
            }
            // SEC
            0x38 => {
                self.sec();
                cycles = 2;
            }
            // SEI
            0x78 => {
                self.sei();
                cycles = 2;
            }
            // CMP
            0xc1 | 0xc5 | 0xc9 | 0xcd | 0xd1 | 0xd5 | 0xd9 | 0xdd => {
                let result = self.alu_address(opcode);
                self.cmp(result.0);
                cycles = result.1;
            }
            // CPX
            0xe0 | 0xe4 | 0xec => {
                let result = self.control_address(opcode);
                self.cpx(result.0);
                cycles = result.1;
            }
            // CPY
            0xc0 | 0xc4 | 0xcc => {
                let result = self.control_address(opcode);
                self.cpy(result.0);
                cycles = result.1;
            }

            // Branch

            // BCC
            0x90 => {
                let result = self.control_address(opcode);
                cycles = self.bcc(result.0);
                cycles += result.1;
            }
            // BCS
            0xb0 => {
                let result = self.control_address(opcode);
                cycles = self.bcs(result.0);
                cycles += result.1;
            }
            // BEQ
            0xf0 => {
                let result = self.control_address(opcode);
                cycles = self.beq(result.0);
                cycles += result.1;
            }
            // BMI
            0x30 => {
                let result = self.control_address(opcode);
                cycles = self.bmi(result.0);
                cycles += result.1;
            }
            // BNE
            0xd0 => {
                let result = self.control_address(opcode);
                cycles = self.bne(result.0);
                cycles += result.1;
            }
            // BPL
            0x10 => {
                let result = self.control_address(opcode);
                cycles = self.bpl(result.0);
                cycles += result.1;
            }
            // BVC
            0x50 => {
                let result = self.control_address(opcode);
                cycles = self.bvc(result.0);
                cycles += result.1;
            }
            // BVS
            0x70 => {
                let result = self.control_address(opcode);
                cycles = self.bvs(result.0);
                cycles += result.1;
            }

            // Jump

            // JMP
            0x4c | 0x6c => {
                let address;
                if opcode == 0x4c {
                    address = self.absolute_address();
                    cycles = 3;
                } else {
                    address = self.indirect_address();
                    cycles = 5;
                }

                self.jmp(address);
            }
            // JSR
            0x20 => {
                let address = self.absolute_address();
                self.jsr(address);
                cycles = 6;
            }
            // RTS
            0x60 => {
                self.rts();
                cycles = 6;
            }
            // RTI
            0x40 => {
                self.rti();
                cycles = 6;
            }

            // System

            // BRK
            0x00 => {
                self.brk();
                cycles = 7;
            }
            // NOP
            0xea => {
                // No op
                cycles = 2;
            }

            _ => panic!("Unrecognized opcode {:?}", opcode)
        }
    }

    // load byte from memory at given address, setting zero and negative flags as appropriate
    fn load(&mut self, address: u16) -> u8 {
        let result = self.memory.fetch(address);
        self.registers.set_zn(result);

        result
    }

    fn load_word(&mut self, address: u16) -> u16 {
        let low = self.memory.fetch(address) as u16;
        let high = self.memory.fetch(address + 1) as u16;
        self.registers.program_counter += 2;
        low | high << 8
    }

    fn store(&mut self, address: u16, value: u8) {
        self.memory.store(address, value);
    }

    pub fn push(&mut self, value: u8) {
        let sp = self.registers.stack_pointer;
        self.memory.store(0x0100 | (sp as u16), value);
        self.registers.stack_pointer -= 1;
    }

    pub fn push_word(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    pub fn pull(&mut self) -> u8 {
        self.registers.stack_pointer += 1;
        self.memory.fetch(0x0100 | (self.registers.stack_pointer as u16))
    }

    pub fn pull_word(&mut self) -> u16 {
        let low = self.pull() as u16;
        let high = self.pull() as u16;

        (high << 8) | low
    }

    // resets all registers
    pub fn reset(&mut self) {
        self.registers.accumulator = 0;
        self.registers.x = 0;
        self.registers.y = 0;
        self.registers.set_flag(INTERRUPT_FLAG, true);
        self.registers.stack_pointer = 0xfd;
        self.registers.program_counter = RESET_ADDR;
    }

    fn shift_left(&mut self, value: u8, lsb: bool) -> u8 {
        let mut result = ((value as u16) << 1) as u8;
        if lsb {
            result |= 1;
        }
        self.registers.set_flag(CARRY_BIT, (value & 0x80) != 0);
        let val = result as u8;
        self.registers.set_zn(val);
        val
    }

    fn shift_right(&mut self, value: u8, msb: bool) -> u8 {
        let mut result = value >> 1;
        if msb {
            result |= 0x80;
        }

        self.registers.set_flag(CARRY_BIT, (value & 0x1) != 0);
        let val = result as u8;
        self.registers.set_zn(val);
        val
    }

    fn transfer(from: u8, to: &mut u8) {
        *to = from;
    }

    fn compare(&mut self, x: u8, y: u8) {
        let result = x as u32 - y as u32;
        self.registers.set_flag(CARRY_BIT, (result & 0x100) == 0);
        self.registers.set_zn(result as u8);
    }

    fn branch(&mut self, address: u16) -> u8 {
        let mut cycles = 1;
        let pc = self.registers.program_counter;
        if !NesMemory::is_same_page(pc, address) {
            cycles += 1;
        }
        self.registers.program_counter = address;
        cycles
    }

    // Instructions

    // Loads a byte into the accumulator register from memory
    fn lda(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.accumulator = result;
    }
    // Load byte into index x register from memory
    fn ldx(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.x = result;
    }
    // Loads a byte into the index y register from memory
    fn ldy(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.y = result;
    }
    // Stores a byte in memory from the accumulator registry
    fn sta(&mut self, address: u16) {
        let a = self.registers.accumulator;
        self.store(address, a);
    }
    // Stores a byte in memory from the index x register
    fn stx(&mut self, address: u16) {
        let x = self.registers.x;
        self.store(address, x);
    }
    // Stores a byte in memory from the index y register
    fn sty(&mut self, address: u16) {
        let y = self.registers.y;
        self.store(address, y);
    }
    // Transfers the value of the accumulator registry into the index x registry
    fn tax(&mut self) {
        let value = self.registers.accumulator;
        self.registers.set_zn(value);

        Cpu::transfer(value, &mut self.registers.x);
    }
    // Transfers the value in the accumulator registry into the index y registry
    fn tay(&mut self) {
        let value = self.registers.accumulator;
        self.registers.set_zn(value);

        Cpu::transfer(value, &mut self.registers.y);
    }
    // Transfers the value in the index x registry into the accumulator registry
    fn txa(&mut self) {
        let value = self.registers.x;
        self.registers.set_zn(value);

        Cpu::transfer(value, &mut self.registers.accumulator);
    }
    // Transfers the value in the index y registry into the accumulator registry
    fn tya(&mut self) {
        let value = self.registers.y;
        self.registers.set_zn(value);

        Cpu::transfer(value, &mut self.registers.accumulator);
    }
    // Transfers the value in the stack pointer registry into the index x registry
    fn tsx(&mut self) {
        let value = self.registers.stack_pointer;
        self.registers.set_zn(value);

        Cpu::transfer(value, &mut self.registers.x);
    }
    // Transfers the value in the index x registry into the stack pointer registry
    fn txs(&mut self) {
        let value = self.registers.x;
        self.registers.set_zn(value);

        Cpu::transfer(value, &mut self.registers.stack_pointer);
    }
    // Pushes the value of the accumulator registry onto the stack
    fn pha(&mut self) {
        let a = self.registers.accumulator;

        self.push(a);
    }
    // Pushes status flags onto the stack
    fn php(&mut self) {
        let mut flags = self.registers.processor_status;
        self.push(flags | BREAK_FLAG);
    }
    // Loads a byte from the stack into the accumulator registry
    fn pla(&mut self) {
        let value = self.pull();
        self.registers.set_zn(value);
        self.registers.accumulator = value;
    }
    // Pulls a byte from the stack and into the processor status registry
    fn plp(&mut self) {
        let value = self.pull();
        self.registers.set_flags(value);
    }
    // Performs a logical AND on a byte from memory and the accumulator's value, and stores the
    // result in the accumulator
    fn and(&mut self, address: u16) {
        let value = self.memory.fetch(address);

        self.registers.set_zn(value);
        self.registers.accumulator &= value;
    }
    // Performs an exclusive OR on a byte of memory and the accumulator's value, and stores the
    // result in the accumulator
    fn eor(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.registers.set_zn(value);
        self.registers.accumulator ^= value;
    }
    // Performs an inclusive OR on a byte of memory and the accumulator's value, and stores the
    // result in the accumulator
    fn ora(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.registers.set_zn(value);
        self.registers.accumulator |= value;
    }
    // Tests if one or more bits is set in the supplied memory location. The accumulator's value
    // is ANDed with the value in memory to set the zero flag, and the value in memory's 6th
    // and 7th bits are used to set the negative and overflow flag respectively.
    fn bit(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let a = self.registers.accumulator;
        self.registers.set_flag(ZERO_FLAG, (value & a) == 0);
        self.registers.set_flag(NEGATIVE_FLAG, (value & 0x80) != 0);
        self.registers.set_flag(OVERFLOW_FLAG, (value & 0x40) != 0);
    }
    // Shifts the contents of memory at the given address left. Effectively multiplies memory
    // contents by two (ignoring two's complement), and sets the carry bit if the result will
    // not fit in 8 bits.
    fn asl(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let result = self.shift_left(value, false);
        self.memory.store(address, result);
    }
    // Shifts the contents of the accumulator registry to the left (see fn asl())
    fn asl_accumulator(&mut self) {
        let a = self.registers.accumulator;
        let result = self.shift_left(a, false);
        Cpu::transfer(result, &mut self.registers.accumulator);
    }
    // Shifts the contents of memory at the given address right.
    fn lsr(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let result = self.shift_right(value, false);
        self.memory.store(address, result);
    }
    // Shifts the contents of the accumulator registry to the right (see fn lsr())
    fn lsr_accumulator(&mut self) {
        let value = self.registers.accumulator;
        let result = self.shift_right(value, false);
        Cpu::transfer(result, &mut self.registers.accumulator);
    }
    // Shifts the contents of memory at the supplied address to the left, setting bit 0 with the
    // current carry flag.
    fn rol(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let carry = self.registers.get_flag(CARRY_BIT);
        let result = self.shift_left(value, carry);
        self.memory.store(address, result);
    }
    // Shift the contents of the accumulator registry to the left, setting bit 0 with the current
    // carry flag.
    fn rol_accumulator(&mut self) {
        let a = self.registers.accumulator;
        let carry = self.registers.get_flag(CARRY_BIT);
        let result = self.shift_left(a, carry);
        Cpu::transfer(result, &mut self.registers.accumulator);
    }
    // Shifts the contents of memory at the supplied address to the right, setting bit 7 with the
    // current carry flag.
    fn ror(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let carry = self.registers.get_flag(CARRY_BIT);
        let result = self.shift_right(value, carry);
        self.memory.store(address, result);
    }
    // Shifts the contents of the accumulator registry to the right, setting bit 7 with the current
    // carry flag.
    fn ror_accumulator(&mut self) {
        let a = self.registers.accumulator;
        let carry = self.registers.get_flag(CARRY_BIT);
        let result = self.shift_right(a, carry);
        Cpu::transfer(result, &mut self.registers.accumulator);
    }
    // Adds the contents of memory at the given address to the value of the accumulator, setting
    // the carry bit if an overflow occurs.
    fn adc(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let mut result = self.registers.accumulator as u32 + value as u32;
        if self.registers.get_flag(CARRY_BIT) {
            result += 1;
        }

        self.registers.set_flag(CARRY_BIT, (result & 0x100) != 0);

        let result = result as u8;
        let a = self.registers.accumulator;
        self.registers.set_flag(OVERFLOW_FLAG,
                                (a ^ value) & 0x80 == 0 && (a ^ result) & 0x80 == 0x80);
        self.registers.set_zn(result);
        self.registers.accumulator = result;
    }
    // Subtracts the contents of memory at the given address from the value of the accumulator,
    // clearing the carry if overflow occurs.
    fn sbc(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let a = self.registers.accumulator;
        let mut result = a as u32 - value as u32;
        if !self.registers.get_flag(CARRY_BIT) {
            result -= 1;
        }

        self.registers.set_flag(CARRY_BIT, (result & 0x100) == 0);

        let result = result as u8;
        self.registers.set_flag(OVERFLOW_FLAG,
                                (a ^ result) & 0x80 != 0 && (a ^ value) & 0x80 == 0x80);
        self.registers.set_zn(result);
        self.registers.accumulator = result;
    }
    // Decreases the value of the given address in memory by one
    fn dec(&mut self, address: u16) {
        let value = self.memory.fetch(address) - 1;
        self.registers.set_zn(value);
        self.memory.store(address, value);
    }
    // Decreases the value of the index x registry by one
    fn dex(&mut self) {
        let value = self.registers.x - 1;
        self.registers.set_zn(value);
        Cpu::transfer(value, &mut self.registers.x);
    }
    // Decreases the value of the index y registry by one
    fn dey(&mut self) {
        let value = self.registers.y - 1;
        self.registers.set_zn(value);
        Cpu::transfer(value, &mut self.registers.y);
    }
    // Adds one to the value of memory at the given address, and stores at the address.
    fn inc(&mut self, address: u16) {
        let value = self.memory.fetch(address) + 1;
        self.registers.set_zn(value);
        self.memory.store(address, value);
    }
    // Adds one to the value of the index x registry
    fn inx(&mut self) {
        let value = self.registers.x + 1;
        self.registers.set_zn(value);
        Cpu::transfer(value, &mut self.registers.x);
    }
    // Adds one to the value of the index y registry
    fn iny(&mut self) {
        let value = self.registers.y + 1;
        self.registers.set_zn(value);
        Cpu::transfer(value, &mut self.registers.y);
    }
    // Sets the carry flag to zero
    fn clc(&mut self) {
        self.registers.set_flag(CARRY_BIT, false);
    }
    // Sets the interrupt disable flag to zero
    fn cli(&mut self) {
        self.registers.set_flag(INTERRUPT_FLAG, false);
    }
    // Sets the overflow flag to zero
    fn clv(&mut self) {
        self.registers.set_flag(OVERFLOW_FLAG, false);
    }
    // Sets the carry flag to one
    fn sec(&mut self) {
        self.registers.set_flag(CARRY_BIT, true);
    }
    // Sets the interrupt disable flag to one
    fn sei(&mut self) {
        self.registers.set_flag(INTERRUPT_FLAG, true);
    }
    // Compares the contents of the accumulator with a value in memory, setting zero and negative
    // flags as appropriate
    fn cmp(&mut self, address: u16) {
        let a = self.registers.accumulator;
        let value = self.memory.fetch(address);
        self.compare(a, value);
    }
    // Compares the contents of the index x registry with a value in memory, setting zero and
    // negative flags as appropriate
    fn cpx(&mut self, address: u16) {
        let x = self.registers.x;
        let value = self.memory.fetch(address);
        self.compare(x, value);
    }
    // Compares the contents of the index x registry with a value in memory, setting zero and
    // negative flags as appropriate
    fn cpy(&mut self, address: u16) {
        let y = self.registers.y;
        let value = self.memory.fetch(address);
        self.compare(y, value);
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // carry flag is not set
    fn bcc(&mut self, address: u16) -> u8 {
        if !self.registers.get_flag(CARRY_BIT) {
            self.branch(address)
        } else {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // carry flag is set
    fn bcs(&mut self, address: u16) -> u8 {
        if self.registers.get_flag(CARRY_BIT) {
            self.branch(address)
        } else {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // zero flag is set
    fn beq(&mut self, address: u16) -> u8 {
        if self.registers.get_flag(ZERO_FLAG) {
            self.branch(address)
        } else {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // negative flag is set
    fn bmi(&mut self, address: u16) -> u8 {
        if self.registers.get_flag(NEGATIVE_FLAG) {
            self.branch(address)
        } else {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // zero flag is not set
    fn bne(&mut self, address: u16) -> u8 {
        if !self.registers.get_flag(ZERO_FLAG) {
            self.branch(address)
        } else {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // negative flag is not set
    fn bpl(&mut self, address: u16) -> u8 {
        if !self.registers.get_flag(NEGATIVE_FLAG) {
            self.branch(address)
        }
        else
        {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // overflow flag is not set
    fn bvc(&mut self, address: u16) -> u8 {
        if !self.registers.get_flag(OVERFLOW_FLAG) {
            self.branch(address)
        } else {
            0
        }
    }
    // Adds the supplied address to the program counter (causing a branch to a new location) if the
    // overflow flag is set
    fn bvs(&mut self, address: u16) -> u8 {
        if self.registers.get_flag(OVERFLOW_FLAG) {
            self.branch(address)
        } else {
            0
        }
    }
    // Sets the program counter to the supplied address
    fn jmp(&mut self, address: u16) {
        self.registers.program_counter = address;
    }
    //
    fn jsr(&mut self, address: u16) {
        let value = self.registers.program_counter - 1;
        self.push_word(value);
    }
    //
    fn rts(&mut self) {
        let value = self.pull_word() + 1;
        self.registers.program_counter = value;
    }
    //
    fn rti(&mut self) {
        let flags = self.pull();
        self.registers.set_flags(flags);
        self.registers.program_counter = self.pull_word();
    }
    //
    fn brk(&mut self) {
        let pc = self.registers.program_counter + 1;
        self.push_word(pc);
        let flags = self.registers.processor_status;
        self.push(flags | BREAK_FLAG);
        self.registers.set_flag(INTERRUPT_FLAG, true);
        let low = self.load(BRK_ADDR) as u16;
        let high = self.load(BRK_ADDR + 1) as u16;

        self.registers.program_counter = (high << 8) | low;
    }

    // Addressing modes

    fn alu_address(&mut self, opcode: u8) -> (u16, u8) {
        let mut cycles: u8;
        let address: u16;
        if opcode & 0x10 == 0 {
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    cycles = 6;
                    address = self.indexed_indirect_address();
                },
                0x01 => {
                    cycles = 5;
                    address = self.zero_page_address();
                },
                0x02 => {
                    cycles = 2;
                    address = self.immediate_address();
                },
                0x03 => {
                    cycles = 4;
                    address = self.absolute_address();
                },
                _ => panic!("unknown alu operation {:#x}", (opcode >> 2) & 0x03)
            }
        } else {
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    cycles = 5;
                    let result = self.indirect_indexed_address();
                    address = result.0;
                    cycles += result.1;
                },
                0x01 => {
                    cycles = 4;
                    address = self.zero_page_indexed_address(Index::X);
                },
                0x02 => {
                    cycles = 4;
                    let result = self.absolute_indexed_address(Index::Y);
                    address = result.0;
                    cycles += result.1;
                },
                0x03 => {
                    cycles = 4;
                    let result = self.absolute_indexed_address(Index::X);
                    address = result.0;
                    cycles += result.1;
                }
                _ => panic!("unknown operation {:#x}", (opcode >> 2) & 0x03)
            }
        }

        (address, cycles)
    }

    fn rmw_address(&mut self, opcode :u8) -> (u16, u8) {
        let mut cycles = 0;
        let address;

        if opcode & 0x10 == 0 {
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    cycles = 2;
                    address = self.immediate_address();
                },
                0x01 => {
                    cycles = 3;
                    address = self.zero_page_address();
                },
                0x02 => {
                    cycles = 2;
                    address = 0; // not used
                },
                0x03 => {
                    cycles = 4;
                    address = self.absolute_address();
                },
                _ => panic!("unknown rmw operation {:#x}", (opcode >> 2) & 0x03)
            };
        } else {
            let index: Index;
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    cycles = 2;
                    address = 0; // not used

                },
                0x01 => {
                    cycles = 4;

                    match opcode & 0xf0 {
                        0x90 => index = Index::Y,
                        0xb0 => index = Index::Y,
                        _ => index = Index::X
                    }

                    address = self.zero_page_indexed_address(index);
                },
                0x02 => {
                    cycles = 2;
                    address = 0; // not used
                },
                0x03 => {
                    cycles = 4;

                    match opcode & 0xf0 {
                        0x90 => index = Index::Y,
                        0xb0 => index = Index::Y,
                        _ => index = Index::X,
                    }

                    let result = self.absolute_indexed_address(index);
                    address = result.0;
                    cycles += result.1;
                },
                _ => panic!("unknown operation {:#x}", (opcode >> 2) & 0x03)
            }
        }

        (address, cycles)
    }

    fn control_address(&mut self, opcode: u8) -> (u16, u8) {
        let mut cycles = 0;
        let address;
        if opcode & 0x10 == 0 {
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    cycles = 2;
                    address = self.immediate_address();
                },
                0x01 => {
                    cycles = 3;
                    address = self.zero_page_address();
                },
                0x02 => {
                    cycles = 4;
                    address = 0; // not used
                },
                0x03 => {
                    cycles = 4;
                    address = self.absolute_address();
                },
                _ => panic!("unknown control operation {:#x}", (opcode >> 2) & 0x03)
            };
        } else {
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    cycles = 2;
                    address = self.relative_address();
                },
                0x01 => {
                    cycles = 4;
                    address = self.zero_page_indexed_address(Index::X);
                },
                0x02 => {
                    cycles = 2;
                    address = 0; // not used
                },
                0x03 => {
                    cycles = 4;
                    let result = self.absolute_indexed_address(Index::X);
                    address = result.0;
                    cycles += result.1;
                },
                _ => panic!("unknown operation {:#x}", (opcode >> 2) & 0x03)
            };
        }

        (address, cycles)
    }

    fn indexed_indirect_address(&mut self) -> u16 {
        let value = self.memory.fetch(self.registers.program_counter);
        let address = (value + self.registers.x) as u16;
        self.registers.program_counter += 1;

        self.load_word(address)
    }

    fn indirect_indexed_address(&mut self) -> (u16, u8) {
        let mut address = self.memory.fetch(self.registers.program_counter) as u16;
        self.registers.program_counter += 1;

        let low = self.memory.fetch(address);
        let high = self.memory.fetch((address + 1) & 0x00ff);

        address = ((high as u16) << 8) | (low as u16);

        let result = address + (self.registers.y as u16);
        let mut cycles = 0;

        if !NesMemory::is_same_page(address, result) {
            cycles += 1;
        }

        (result, cycles)
    }

    fn zero_page_address(&mut self) -> u16 {
        let result = self.memory.fetch(self.registers.program_counter) as u16;
        self.registers.program_counter += 1;

        result
    }

    fn immediate_address(&mut self) -> u16 {
        let result = self.registers.program_counter;
        self.registers.program_counter += 1;
        result
    }

    // Retrieves two bytes of memory, and increments the program counter twice. Returns the
    // value interpreted as little endian
    fn absolute_address(&mut self) -> u16 {
        let pc = self.registers.program_counter;
        self.load_word(pc)
    }

    // Adds the contents of the register pertaining to the supplied index to the address
    // in the program counter, effectively using the supplied index register as an offset.
    // Increments the program counter and returns the resulting address.
    fn zero_page_indexed_address(&mut self, index: Index) -> u16 {
        let value = self.memory.fetch(self.registers.program_counter);
        let result = (value + self.registers.register_from_index(index)) as u16;
        self.registers.program_counter += 1;

        result
    }

    // Add the contents of the register pertaining to index to an absolute address, effectively
    // using the supplied index register as an offset. Also increments the program counter.
    // Returns the resulting address and the number of cycles this operation should take.
    fn absolute_indexed_address(&mut self, index: Index) -> (u16, u8) {
        let pc = self.registers.program_counter;
        let address = self.load_word(pc);
        let result = address + (self.registers.register_from_index(index) as u16);

        let mut cycles = 0;
        if !NesMemory::is_same_page(address, result) {
            cycles = 1;
        }

        (result, cycles)
    }

    fn relative_address(&mut self) -> u16 {
        let value = self.memory.fetch(self.registers.program_counter) as u16;
        self.registers.program_counter += 1;
        let offset = if value > 0x7f {
            !(0x0100 - value)
        } else {
            value
        };

        let result = self.registers.program_counter + offset;
        result
    }

    fn indirect_address(&mut self) -> u16 {
        let pc = self.registers.program_counter;
        let addr = self.load_word(pc);
        self.registers.program_counter += 2;

        // 6502 has a bug where it only increments the high byte instead
        // of the entire 16-bit address.
        let low_val = self.memory.fetch(addr);
        let high_val = self.memory.fetch((addr & 0xff00) | ((addr + 1) & 0x00ff));

        println!("{:x}", low_val);
        println!("{:x}", high_val);

        (high_val as u16) << 8 | low_val as u16
    }
}

#[derive(Default, Debug)]
pub struct Registers {
    // (A) Accumulator, arithmetic/logic instructions
    pub accumulator: u8,
    // (X/Y) index registers (used for indirect addressing and counters/indexes)
    pub x: u8,
    pub y: u8,
    // (SP) stack pointer (stores least sig bit of top of the stack)
    pub stack_pointer: u8,
    // (PC) program counter (only 16 bit register, points to next instruction to execute)
    pub program_counter: u16,
    // (P) processor status (indicate results of last arithmetic and logic instructions,
    // indicates break/interrupts)
    pub processor_status: u8
}

impl Registers {
    fn register_from_index(&self, index: Index) -> u8 {
        match index {
            Index::X => self.x,
            Index::Y => self.y
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        (self.processor_status & flag) != 0
    }

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.processor_status |= flag;
        } else {
            self.processor_status &= !flag;
        }
    }

    fn set_flags(&mut self, value: u8) {
        // apparently status flags get mangled in some way relating to the unused 5th bit
        self.processor_status = (value | 0x30) - 0x10;
    }

    fn set_zn(&mut self, value: u8) {
        self.set_flag(ZERO_FLAG, value == 0);
        self.set_flag(NEGATIVE_FLAG, (value & 0x80) != 0);
    }
}

#[derive(Debug)]
enum Index {
    X,
    Y
}