use super::status::Status;
use super::memory;
use super::memory::Memory;
use super::memory::NesMemory;
use std::cell::RefCell;

static CYCLE_TABLE: [u8; 256] = [
    /*0x00*/ 7,6,2,8,3,3,5,5,3,2,2,2,4,4,6,6,
    /*0x10*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x20*/ 6,6,2,8,3,3,5,5,4,2,2,2,4,4,6,6,
    /*0x30*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x40*/ 6,6,2,8,3,3,5,5,3,2,2,2,3,4,6,6,
    /*0x50*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x60*/ 6,6,2,8,3,3,5,5,4,2,2,2,5,4,6,6,
    /*0x70*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0x80*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
    /*0x90*/ 2,6,2,6,4,4,4,4,2,5,2,5,5,5,5,5,
    /*0xA0*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
    /*0xB0*/ 2,5,2,5,4,4,4,4,2,4,2,4,4,4,4,4,
    /*0xC0*/ 2,6,2,8,3,3,5,5,2,2,2,2,4,4,6,6,
    /*0xD0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
    /*0xE0*/ 2,6,3,8,3,3,5,5,2,2,2,2,4,4,6,6,
    /*0xF0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
];

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
    memory: NesMemory
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            memory: NesMemory::new()
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
            },

            // LDX
            0xa2 | 0xa6 | 0xa3 | 0xb2 | 0xb6 | 0xbe => {
                let result = self.rmw_address(opcode);
                self.ldx(result.0);
                cycles = result.1;
            },

            // LDY
            0xa0 | 0xa4 | 0xac | 0xb4 | 0xbc => {
                let result = self.control_address(opcode);
                self.ldy(result.0);
                cycles = result.1;
            },

            // Storing

            // STA
            0x81 | 0x85 | 0x8d | 0x91 | 0x95 | 0x99 | 0x9d => {
                let result = self.alu_address(opcode);
                self.sta(result.0);
                cycles = result.1;
            },

            // STX
            0x86 | 0x8e | 0x96 => {
                let result = self.rmw_address(opcode);
                self.stx(result.0);
                cycles = result.1;
            },

            // STY
            0x84 | 0x8c | 0x94 => {
                let result = self.control_address(opcode);
                self.sty(result.0);
                cycles = result.1;
            },

            // Transferring

            // TAX
            0xaa => {
                cycles = 2;
                self.tax();
            },

            // TAY
            0xa8 => {
                cycles = 2;
                self.tay();
            },

            // TXA
            0x8a => {
                cycles = 2;
                self.txa();
            }

            // TYA
            0x98 => {
                cycles = 2;
                self.tya();
            },

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


            _ => panic!("Unrecognized opcode {:#x}", opcode)
        }

        cycles += CYCLE_TABLE[opcode as usize];
    }

    // load byte from memory at given address, setting zero and negative flags as appropriate
    fn load(&mut self, address: u16) -> u8 {
        let result = self.memory.fetch(address);
        self.registers.processor_status.set_negative(result);
        self.registers.processor_status.set_zero(result);

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

    fn set_zero_and_negative(&mut self, value: u8) {
        self.registers.processor_status.set_zero(value);
        self.registers.processor_status.set_negative(value);
    }

    fn transfer(&self, from: u8, to: &mut u8) {
        *to = from;
    }

    // Instructions

    // Load byte into accumulator register from memory
    fn lda(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.accumulator = result;
    }

    // Load byte into index x register from memory
    fn ldx(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.index_register_y = result;
    }

    // Load byte into index y register from memory
    fn ldy(&mut self, address: u16) {
        let result = self.load(address);
        self.registers.index_register_y = result;
    }

    // Store byte in memory from accumulator
    fn sta(&mut self, address: u16) {
        let a = self.registers.accumulator;
        self.store(address, a);
    }

    // Store byte in memory from index x register
    fn stx(&mut self, address: u16) {
        let x = self.registers.index_register_x;
        self.store(address, x);
    }

    // Store byte in memory from index y register
    fn sty(&mut self, address: u16) {
        let y = self.registers.index_register_y;
        self.store(address, y);
    }

    fn tax(&mut self) {
        let value = self.registers.accumulator;
        self.set_zero_and_negative(value);

        let mut x = self.registers.index_register_x;

        self.transfer(value, &mut x);
    }

    fn tay(&mut self) {
        let value = self.registers.accumulator;
        self.set_zero_and_negative(value);

        let mut y = self.registers.index_register_y;

        self.transfer(value, &mut y);
    }

    fn txa(&mut self) {
        let value = self.registers.index_register_x;
        self.set_zero_and_negative(value);

        let mut a = self.registers.accumulator;

        self.transfer(value, &mut a);
    }

    fn tya(&mut self) {
        let value = self.registers.index_register_y;
        self.set_zero_and_negative(value);

        let mut a = self.registers.accumulator;

        self.transfer(value, &mut a);
    }

    fn tsx(&mut self) {
        let value = self.registers.stack_pointer;
        self.set_zero_and_negative(value);

        let mut x = self.registers.index_register_x;

        self.transfer(value, &mut x);
    }

    fn txs(&mut self) {
        let value = self.registers.index_register_x;
        self.set_zero_and_negative(value);

        let mut sp = self.registers.stack_pointer;

        self.transfer(value, &mut sp);
    }

    fn pha(&mut self) {
        let a = self.registers.accumulator;

        self.push(a);
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
    processor_status: Status
}

impl Registers {
    fn register_from_index(&self, index: Index) -> u8 {
        match index {
            Index::X => self.index_register_x,
            Index::Y => self.index_register_y
        }
    }
}

#[derive(Debug)]
enum Index {
    X,
    Y
}