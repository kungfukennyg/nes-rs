use super::memory;
use super::memory::Memory;
use super::memory::NesMemory;
use std::cell::RefCell;

const CARRY_BIT: u8 = 0;
const ZERO_FLAG: u8 = 1;
const INTERRUPT_FLAG: u8 = 2;
const BREAK_FLAG: u8 = 4;
const OVERFLOW_FLAG: u8 = 6;
const NEGATIVE_FLAG: u8 = 7;

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
    memory: NesMemory,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            memory: NesMemory::new(),
        }
    }

    pub fn execute_instruction(&mut self, value: u8) {
        let instruction = self.memory.fetch(self.registers.program_counter);
        let opcode = instruction >> 6;
        let mut cycles = 0;

        match opcode {

            // Loading

            // LDA
            0xa1 | 0xa5 | 0xa9 | 0xad | 0xb1 | 0xb5 | 0xb9 | 0xbd => {
                let result = self.alu_address(opcode);
                self.lda(result.0);
                cycles = result.1;
            }
            // LDX
            0xa2 | 0xa6 | 0xa3 | 0xb2 | 0xb6 | 0xbe => {
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

            // Storing

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

            // Transferring

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
            0x01 | 0x05 | 0x09 | 0x04 | 0x11 | 0x15 | 0x19 | 0x1d => {
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
            0xe6 | 0xf6 | 0xfe => {
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


            _ => panic!("Unrecognized opcode {:#x}", opcode)
        }
    }

    // load byte from memory at given address, setting zero and negative flags as appropriate
    fn load(&mut self, address: u16) -> u8 {
        let result = self.memory.fetch(address);
        self.registers.set_zn(result);

        result
    }

    fn store(&mut self, address: u16, value: u8) {
        self.memory.store(address, value);
    }

    fn push(&mut self, value: u8) {
        let sp = self.registers.stack_pointer;
        self.memory.store(0x0100 | (sp as u16), value);
        self.registers.stack_pointer -= 1;
    }

    fn pull(&mut self) -> u8 {
        self.registers.stack_pointer += 1;
        self.memory.fetch(0x0100 | (self.registers.stack_pointer as u16))
    }

    fn shift_left(&mut self, value: u8, lsb: bool) -> u8 {
        let mut result = value << 1;
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
            result |= 1;
        }

        self.registers.set_flag(CARRY_BIT, (value & 0x1) != 0);
        let val = result as u8;
        self.registers.set_zn(val);
        val
    }

    fn transfer(&self, from: u8, to: &mut u8) {
        *to = from;
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
        self.registers.index_register_y = result;
    }
    // Loads a byte into the index y register from memory
    fn ldy(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.index_register_y = result;
    }
    // Stores a byte in memory from the accumulator registry
    fn sta(&mut self, address: u16) {
        let a = self.registers.accumulator;
        self.store(address, a);
    }
    // Stores a byte in memory from the index x register
    fn stx(&mut self, address: u16) {
        let x = self.registers.index_register_x;
        self.store(address, x);
    }
    // Stores a byte in memory from the index y register
    fn sty(&mut self, address: u16) {
        let y = self.registers.index_register_y;
        self.store(address, y);
    }
    // Transfers the value of the accumulator registry into the index x registry
    fn tax(&mut self) {
        let value = self.registers.accumulator;
        self.registers.set_zn(value);

        let mut x = self.registers.index_register_x;

        self.transfer(value, &mut x);
    }
    // Transfers the value in the accumulator registry into the index y registry
    fn tay(&mut self) {
        let value = self.registers.accumulator;
        self.registers.set_zn(value);

        let mut y = self.registers.index_register_y;

        self.transfer(value, &mut y);
    }
    // Transfers the value in the index x registry into the accumulator registry
    fn txa(&mut self) {
        let value = self.registers.index_register_x;
        self.registers.set_zn(value);

        let mut a = self.registers.accumulator;

        self.transfer(value, &mut a);
    }
    // Transfers the value in the index y registry into the accumulator registry
    fn tya(&mut self) {
        let value = self.registers.index_register_y;
        self.registers.set_zn(value);

        let mut a = self.registers.accumulator;

        self.transfer(value, &mut a);
    }
    // Transfers the value in the stack pointer registry into the index x registry
    fn tsx(&mut self) {
        let value = self.registers.stack_pointer;
        self.registers.set_zn(value);

        let mut x = self.registers.index_register_x;

        self.transfer(value, &mut x);
    }
    // Transfers the value in the index x registry into the stack pointer registry
    fn txs(&mut self) {
        let value = self.registers.index_register_x;
        self.registers.set_zn(value);

        let mut sp = self.registers.stack_pointer;

        self.transfer(value, &mut sp);
    }
    // Pushes the value of the accumulator registry onto the stack
    fn pha(&mut self) {
        let a = self.registers.accumulator;

        self.push(a);
    }
    // Pushes status flags onto the stack
    fn php(&mut self) {
        let mut flags = self.registers.processor_status;
        flags |= 1 << BREAK_FLAG; // set break flag
        self.push(flags);
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
        let mut a = self.registers.accumulator;
        let result = self.shift_left(a, false);
        self.transfer(result, &mut a);
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
        let mut a = self.registers.accumulator;
        self.transfer(result, &mut a);
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
        let mut a = self.registers.accumulator;
        let carry = self.registers.get_flag(CARRY_BIT);
        let result = self.shift_left(a, carry);
        self.transfer(result, &mut a);
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
        let mut a = self.registers.accumulator;
        let carry = self.registers.get_flag(CARRY_BIT);
        let result = self.shift_left(a, carry);
        self.transfer(result, &mut a);
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
        let value = self.registers.index_register_x - 1;
        self.registers.set_zn(value);
        let mut x = self.registers.index_register_x;
        self.transfer(value, &mut x);
    }
    // Decreases the value of the index y registry by one
    fn dey(&mut self) {
        let value = self.registers.index_register_y - 1;
        self.registers.set_zn(value);
        let mut y = self.registers.index_register_y;
        self.transfer(value, &mut y);
    }
    // Adds one to the value of memory at the given address, and stores at the address.
    fn inc(&mut self, address: u16) {
        let value = self.memory.fetch(address) + 1;
        self.registers.set_zn(value);
        self.memory.store(address, value);
    }
    // Adds one to the value of the index x registry
    fn inx(&mut self) {
        let value = self.registers.index_register_x + 1;
        self.registers.set_zn(value);
        let mut x = self.registers.index_register_x;
        self.transfer(value, &mut x);
    }
    // Adds one to the value of the index y registry
    fn iny(&mut self) {
        let value = self.registers.index_register_y + 1;
        self.registers.set_zn(value);
        let mut y = self.registers.index_register_y;
        self.transfer(value, &mut y);
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
        let address = (value + self.registers.index_register_y) as u16;
        self.registers.program_counter += 1;

        let low = self.memory.fetch(address);
        let high = self.memory.fetch((address + 1) & 0x00ff);

        ((high as u16) << 8) | (low as u16)
    }

    fn indirect_indexed_address(&mut self) -> (u16, u8) {
        let mut address = self.memory.fetch(self.registers.program_counter) as u16;
        self.registers.program_counter += 1;

        let low = self.memory.fetch(address);
        let high = self.memory.fetch((address + 1) & 0x00ff);

        address = ((high as u16) << 8) | (low as u16);

        let result = address + (self.registers.index_register_y as u16);
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
        let low = self.memory.fetch(self.registers.program_counter);
        let high = self.memory.fetch(self.registers.program_counter + 1);
        self.registers.program_counter += 2;

        ((high as u16) << 8) | (low as u16)
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
        let low = self.memory.fetch(self.registers.program_counter);
        let high = self.memory.fetch(self.registers.program_counter + 1);
        self.registers.program_counter += 2;

        let address = ((high as u16) << 8) | (low as u16);
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
}

#[derive(Default, Debug)]
struct Registers {
    // (A) Accumulator, arithmetic/logic instructions
    accumulator: u8,
    // (X/Y) index registers (used for indirect addressing and counters/indexes)
    index_register_x: u8,
    index_register_y: u8,
    // (SP) stack pointer (stores least sig bit of top of the stack)
    stack_pointer: u8,
    // (PC) program counter (only 16 bit register, points to next instruction to execute)
    program_counter: u16,
    // (P) processor status (indicate results of last arithmetic and logic instructions,
    // indicates break/interrupts)
    processor_status: u8
}

impl Registers {
    fn register_from_index(&self, index: Index) -> u8 {
        match index {
            Index::X => self.index_register_x,
            Index::Y => self.index_register_y
        }
    }

    fn get_flag(&self, flag: u8) -> bool {
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