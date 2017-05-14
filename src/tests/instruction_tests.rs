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

    // LDX

    #[test]
    fn test_ldx_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa2);
        cpu.memory.store(0x0101, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);
    }

    #[test]
    fn test_ldx_zero_page_x_and_y() {
        let mut cpu = Cpu::new();

        // X
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x100, 0xa6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_y = 0x01;

        cpu.memory.store(0x0100, 0xb6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);
    }

    #[test]
    fn test_ldx_absolute_x_and_y() {
        let mut cpu = Cpu::new();

        // X
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xae);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_y = 1;

        cpu.memory.store(0x0100, 0xbe);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);
    }

    // LDY

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa0);
        cpu.memory.store(0x0101, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_y == 0xff);
    }

    #[test]
    fn test_ldy_zero_page_y_and_x() {
        let mut cpu = Cpu::new();

        // Y
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa4);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_y == 0xff);

        cpu.memory.reset();

        // X
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_x = 0x01;

        cpu.memory.store(0x0100, 0xb4);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_y == 0xff);
    }

    #[test]
    fn test_ldy_absolute_y_and_x()
    {
        let mut cpu = Cpu::new();

        // Y
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xac);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_y == 0xff);

        cpu.memory.reset();

        // X
        cpu.registers.program_counter = 0x0100;
        cpu.registers.index_register_x = 1;

        cpu.memory.store(0x0100, 0xbc);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_y == 0xff);
    }

    // STA

    #[test]
    fn test_sta_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x85);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_sta_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x95);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_sta_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x8d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_sta_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x9d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_sta_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x99);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_sta_indirect_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x81);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x87);
        cpu.memory.store(0x0086, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0087) == 0xff);
    }

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x91);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0087) == 0xff);
    }

    // STX

    #[test]
    fn test_stx_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x86);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_stx_zero_page_y() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_x = 0xff;
        cpu.registers.index_register_y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x96);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_stx_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x8e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    // STY

    #[test]
    fn test_sty_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x84);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_sty_zero_page_y() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_y = 0xff;
        cpu.registers.index_register_x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x94);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_sty_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x8c);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    // TAX

    #[test]
    fn test_tax()
    {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xaa);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);
    }

    // TAY

    #[test]
    fn test_tay() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa8);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_y == 0xff);
    }

    // TXA

    #[test]
    fn test_txa() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x8a);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    // TYA

    #[test]
    fn test_tya() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x98);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    // TSX

    #[test]
    fn test_tsx() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;
        cpu.registers.stack_pointer = 0xff;

        cpu.memory.store(0x0100, 0xba);

        cpu.execute_instruction();

        assert!(cpu.registers.index_register_x == 0xff);
    }

    // TXS

    #[test]
    fn test_txs() {
        let mut cpu = Cpu::new();

        cpu.registers.index_register_x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x9a);

        cpu.execute_instruction();

        assert!(cpu.registers.stack_pointer == 0xff);
    }

    // PHA

    #[test]
    fn test_pha() {
        let mut cpu = Cpu::new();

        // reset registers (just so stack pointer isn't allowed to underflow)
        cpu.reset();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x48);

        cpu.execute_instruction();

        assert!(cpu.pull() == 0xff);
    }

    // PHP

    #[test]
    fn test_php() {
        let mut cpu = Cpu::new();

        // see test_pha
        cpu.reset();

        cpu.registers.processor_status = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x08);

        cpu.execute_instruction();

        assert!(cpu.pull() == 0xff);
    }

    // PLA

    #[test]
    fn test_pla() {
        let mut cpu = Cpu::new();

        // see test_pha
        cpu.reset();

        cpu.registers.program_counter = 0x0100;
        cpu.push(0xff);

        cpu.memory.store(0x0100, 0x68);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    // PLP

    #[test]
    fn test_plp() {
        let mut cpu = Cpu::new();

        // set test_pha
        cpu.reset();

        cpu.registers.program_counter = 0x0100;
        cpu.push(0xff);

        cpu.memory.store(0x0100, 0x28);

        cpu.execute_instruction();

        assert!(cpu.registers.processor_status == 0xef);
    }

    // AND

    #[test]
    fn test_and_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x29);
        cpu.memory.store(0x0101, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    #[test]
    fn test_and_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x25);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    #[test]
    fn test_and_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x0ff;
        cpu.registers.index_register_x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x35);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    #[test]
    fn test_and_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x2d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    #[test]
    fn test_and_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_x = 1;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x3d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    #[test]
    fn test_and_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_y = 1;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x39);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    #[test]
    fn test_and_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.index_register_y = 1;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x31);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

}