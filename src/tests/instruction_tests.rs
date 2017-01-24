#[cfg(test)]
mod tests {
    use super::*;
    use nes::cpu::Cpu;
    use nes::memory::Memory;
    use nes::memory::NesMemory;

    // LDA
    #[test]
    fn test_lda_immediate() {
        let mut cpu: Cpu = Cpu::new();

        cpu.registers.program_counter = 0x010;

        cpu.memory.store(0x010, 0xa9);
        cpu.memory.store(0x011, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }
    #[test]
    fn test_lda_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa5);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }
    #[test]
    fn test_lda_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_x = 0x01;

        cpu.memory.store(0x0100, 0xb5);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }
    #[test]
    fn test_lda_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xad);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }
    #[test]
    fn test_lda_absolute_x_and_y() {
        let mut cpu = Cpu::new();

        // X
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_x = 1;

        cpu.memory.store(0x0100, 0xbd);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_y = 1;

        cpu.memory.store(0x0100, 0xb9);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff)
    }
    #[test]
    fn test_lda_indirect_x_and_y() {
        let mut cpu = Cpu::new();

        // X
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_x = 1;

        cpu.memory.store(0x0100, 0xa1);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x87);
        cpu.memory.store(0x0086, 0x00);
        cpu.memory.store(0x0087, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_y = 1;

        cpu.memory.store(0x0100, 0xb1);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }
}