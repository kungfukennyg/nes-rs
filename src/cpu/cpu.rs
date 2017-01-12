use super::status::Status;

#[derive(Debug)]
pub struct Cpu {
    registers: Registers
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default()
        }
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
    // (P) processor status (indicate results of last arithmetic and logic instructions, indicates break/interrupts)
    processor_status: Status
}