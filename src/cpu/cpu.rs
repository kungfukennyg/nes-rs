use super::status::Status;
use super::memory;
use super::memory::Memory;
use super::memory::NesMemory;
use super::instruction::InstructionTable;
use super::instruction::Instruction;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Cpu<'a> {
    registers: Registers,
    memory: NesMemory,
    instruction_table: RefCell<InstructionTable<'a>>
}

impl<'a> Cpu<'a> {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            memory: NesMemory::new(),
            instruction_table: RefCell::new(InstructionTable::new())
        }
    }

    pub fn run(&self) {
        let instruction = self.instruction_table.borrow_mut().exec_instruction(0xa1, self);

    }

    pub fn execute_instruction(&mut self, value: u8) {
        let instruction = self.memory.fetch(self.registers.program_counter);
        let opcode = instruction >> 6;

        println!("{:?}", opcode);
    }

    pub fn load(&self, address: u16, register: u8) {

    }

    pub fn lda(&mut self, address: u16) {

    }

    pub fn alu_address(&mut self, opcode: u8) -> (u16, u8) {
        let cycles: u8;
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
                _ => panic!("TODO")
            }
        } else {
            match (opcode >> 2) & 0x03 {
                0x00 => {
                    let result = self.indirect_indexed_address();
                    cycles = result.1;
                    address = result.0;
                },
                0x01 => {
                    cycles = 4;
                    address = self.zero_page_indexed_address(Index::X);
                },
                _ => panic!("TODO")
            }
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

    fn absolute_address(&mut self) -> u16 {
        let low = self.memory.fetch(self.registers.program_counter);
        let high = self.memory.fetch(self.registers.program_counter + 1);
        self.registers.program_counter += 2;

        ((high as u16) << 8) | (low as u16)
    }

    fn zero_page_indexed_address(&mut self, index: Index) -> u16 {
        let value = self.memory.fetch(self.registers.program_counter);
        let result = (value + self.registers.register_from_index(index)) as u16;
        self.registers.program_counter += 1;

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