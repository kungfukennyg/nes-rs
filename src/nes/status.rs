const CARRY_BIT: u8 = 1 << 0;
const ZERO_FLAG: u8 = 1 << 1;
const INTERRUPT_FLAG: u8 = 1 << 2;
const DECIMAL_FLAG: u8 = 1 << 3;
const BREAK_FLAG: u8 = 1 << 4;
const OVERFLOW_FLAG: u8 = 1 << 6;
const NEGATIVE_FLAG: u8 = 1 << 7;

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
    // (N) negative flag (set if sign bit on last instruction
}

impl Status {

    pub fn set_negative(&mut self, value: u8) {
        self.zero_flag = (value & 0x80) == 0;
    }

    pub fn set_zero(&mut self, value: u8) {
        self.carry_flag = value == 0;
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry_flag = value;
    }
}