use std::fmt;
use nes::rom::Rom;
use core::borrow::{Borrow, BorrowMut};
use nes::mapper::{create_mapper, Mapper};

const DEFAULT_MEMORY_SIZE: u32 = 65536; // change to 2048;

pub trait Memory {
    fn reset(&mut self);
    fn load(&self, address: u16) -> u8;
    fn store(&mut self, address: u16, value: u8);
    fn is_same_page(address1: u16, address2: u16) -> bool;
}

pub struct NesMemory {
    memory: [u8; DEFAULT_MEMORY_SIZE as usize],
    mapper: Box<Mapper>,
}

impl NesMemory {
    pub fn new(rom: Box<Rom>) -> Self {
        let mapper = create_mapper(rom);
        NesMemory {
            memory: [0; DEFAULT_MEMORY_SIZE as usize],
            mapper
        }
    }
}

impl Memory for NesMemory {
    // set memory at every address to 0
    fn reset(&mut self) {
        for x in 0..DEFAULT_MEMORY_SIZE {
            self.memory[x as usize] = 0;
        }
    }

    // retrieve value from memory at address
    fn load(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.memory[address as usize]
        } else if address < 0x4000 {
            // ppu
            self.memory[address as usize]
        } else if address == 0x4016 {
            // input
            self.memory[address as usize]
        } else if address <= 0x4018 {
            // apu
            self.memory[address as usize]
        } else if address < 0x6000 {
            // some mappers?
            self.memory[address as usize]
        } else {
            let mapper: &Mapper = self.mapper.borrow();
            mapper.prg_load(address)
        }
    }

    // stores value in the given address and returns the previous value
    fn store(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.memory[address as usize] = value;
        } else if address < 0x4000 {
            // ppu

        } else if address == 0x4016 {
            // input

        } else if address <= 0x4018 {
            // apu

        } else if address < 0x6000 {
            // some mappers?

        } else {
            let mut mapper: &mut Mapper = self.mapper.borrow_mut();
            mapper.prg_store(address, value);
        }
    }

    /// returns true if two addresses' higher bits are the same
    /// , aka if they are located in the same page in memory.
    /// i.e. 0x0101 and 0x0103 are on the same page,
    /// but 0x0101 0x0202 are not.
    fn is_same_page(address1: u16, address2: u16) -> bool {
        (address1 ^ address2) >> 8 == 0
    }
}

impl fmt::Debug for NesMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO nes memory fmt")
    }
}