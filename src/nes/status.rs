const CARRY_BIT: u8 = 0;
const ZERO_FLAG: u8 = 1;
const INTERRUPT_FLAG: u8 = 2;
const BREAK_FLAG: u8 = 4;
const OVERFLOW_FLAG: u8 = 6;
const NEGATIVE_FLAG: u8 = 7;

#[derive(Default, Debug)]
pub struct Status {
    // (C) carry flag (set if last instruction resulted in over/under flow)
    // allows calculations on numbers longer than 8 bits
    carry_flag: bool,
    // (Z) zero flag (set if result of last instruction was 0)
    zero_flag: bool,
    // (I) interrupt disable (used to ignore IRQs)
    interrupt_flag: bool,
    // (B) break flag (used to indicate that a break (BRK) has executed, causing an IRQ)
    break_flag: bool,
    // (V) overflow flag (set if an invalid two's complement result was obtained from previous
    // instruction)
    overflow_flag: bool,
    // (N) negative flag (set if sign bit on last instruction is set)
    negative_flag: bool
}

impl Status {

    pub fn new() -> Self {
        Status {
            carry_flag: false,
            zero_flag: false,
            interrupt_flag: false,
            break_flag: false,
            overflow_flag: false,
            negative_flag: false
        }
    }

    pub fn set_negative(&mut self, value: u8) {
        self.negative_flag = (value & 0x80) == 0;
    }

    pub fn set_zero(&mut self, value: u8) {
        self.zero_flag = value == 0;
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry_flag = value;
    }

    pub fn value(&self) -> u8 {
        let mut value = 0;

        value |= (self.carry_flag as u8) << CARRY_BIT;
        value |= (self.zero_flag as u8) << ZERO_FLAG;
        value |= (self.interrupt_flag as u8) << INTERRUPT_FLAG;
        value |= (self.break_flag as u8) << BREAK_FLAG;
        value |= (self.overflow_flag as u8) << OVERFLOW_FLAG;
        value |= (self.negative_flag as u8) << NEGATIVE_FLAG;

        value
    }
}