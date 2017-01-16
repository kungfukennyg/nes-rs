use std::fmt;

const DEFAULT_MEMORY_SIZE: u32 = 65536;

pub trait Memory {
    fn reset(&mut self);
    fn fetch(&self, address: u16) -> u8;
    fn store(&mut self, address: u16, value: u8) -> u8;
}

pub struct NesMemory {
    memory: [u8; DEFAULT_MEMORY_SIZE as usize]
}

impl NesMemory {
    pub fn new() -> Self {
        NesMemory {
            memory: [0; DEFAULT_MEMORY_SIZE as usize]
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
    fn fetch(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    // stores value in the given address and returns the previous value
    fn store(&mut self, address: u16, value: u8) -> u8 {
        let old = self.fetch(address);
        self.memory[address as usize] = value;
        old
    }
}

impl fmt::Debug for NesMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO nes memory fmt")
    }
}