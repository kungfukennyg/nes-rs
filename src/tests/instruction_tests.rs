#[cfg(test)]
mod tests {
    use super::*;
    use nes::cpu;
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
        cpu.registers.x = 0x01;

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
        cpu.registers.x = 1;

        cpu.memory.store(0x0100, 0xbd);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.y = 1;

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
        cpu.registers.x = 1;

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
        cpu.registers.y = 1;

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

        assert!(cpu.registers.x == 0xff);
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

        assert!(cpu.registers.x == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.y = 0x01;

        cpu.memory.store(0x0100, 0xb6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.x == 0xff);
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

        assert!(cpu.registers.x == 0xff);

        cpu.memory.reset();

        // Y
        cpu.registers.program_counter = 0x0100;
        cpu.registers.y = 1;

        cpu.memory.store(0x0100, 0xbe);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.x == 0xff);
    }

    // LDY

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa0);
        cpu.memory.store(0x0101, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.y == 0xff);
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

        assert!(cpu.registers.y == 0xff);

        cpu.memory.reset();

        // X
        cpu.registers.program_counter = 0x0100;
        cpu.registers.x = 0x01;

        cpu.memory.store(0x0100, 0xb4);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.y == 0xff);
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

        assert!(cpu.registers.y == 0xff);

        cpu.memory.reset();

        // X
        cpu.registers.program_counter = 0x0100;
        cpu.registers.x = 1;

        cpu.memory.store(0x0100, 0xbc);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.y == 0xff);
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
        cpu.registers.x = 0x01;
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
        cpu.registers.x = 0x01;
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
        cpu.registers.y = 0x01;
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
        cpu.registers.x = 0x01;
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
        cpu.registers.y = 0x01;
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

        cpu.registers.x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x86);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_stx_zero_page_y() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x96);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_stx_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
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

        cpu.registers.y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x84);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_sty_zero_page_y() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x94);
        cpu.memory.store(0x0101, 0x84);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_sty_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xff;
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

        assert!(cpu.registers.x == 0xff);
    }

    // TAY

    #[test]
    fn test_tay() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xa8);

        cpu.execute_instruction();

        assert!(cpu.registers.y == 0xff);
    }

    // TXA

    #[test]
    fn test_txa() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x8a);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    // TYA

    #[test]
    fn test_tya() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xff;
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

        assert!(cpu.registers.x == 0xff);
    }

    // TXS

    #[test]
    fn test_txs() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
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
        cpu.registers.x = 0x01;
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
        cpu.registers.x = 1;
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
        cpu.registers.y = 1;
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
        cpu.registers.y = 1;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x31);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x0f);
    }

    // EOR

    #[test]
    fn test_eor_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x45);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x45);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x55);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_absolute() {
        let mut cpu = Cpu::new();
        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x4d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x5d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x59);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_indirect_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x41);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x87);
        cpu.memory.store(0x0086, 0x00);
        cpu.memory.store(0x0087, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    #[test]
    fn test_eor_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x51);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xf0);
    }

    // ORA

    #[test]
    fn test_ora_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x09);
        cpu.memory.store(0x0101, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    #[test]
    fn test_ora_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x05);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    #[test]
    fn test_ora_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x015);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    #[test]
    fn test_ora_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x0d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    #[test]
    fn test_ora_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x1d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    #[test]
    fn test_ora_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x19);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    #[test]
    fn test_ora_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xf0;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x11);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0x0f);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0xff);
    }

    // BIT

    #[test]
    fn test_bit_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x24);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x7f);

        cpu.execute_instruction();

        assert!(cpu.registers.processor_status & (cpu::BREAK_FLAG) == 0);
    }

    #[test]
    fn test_bit_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x2c);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x7f);

        cpu.execute_instruction();

        assert!(cpu.registers.processor_status & (cpu::BREAK_FLAG) == 0);
    }

    // ADC

    #[test]
    fn test_adc_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x69);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x65);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x75);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x6d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x7d);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x79);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_indirect_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x61);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x87);
        cpu.memory.store(0x0086, 0x00);
        cpu.memory.store(0x0087, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    #[test]
    fn test_adc_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x01;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x71);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x03);
    }

    // SBC

    #[test]
    fn test_sbc_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe9);
        cpu.memory.store(0x0101, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe5);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xf5);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xed);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xfd);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xf9);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_indirect_x() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe1);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x87);
        cpu.memory.store(0x0086, 0x00);
        cpu.memory.store(0x0087, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_sbc_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x02;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xf1);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    // CMP

    #[test]
    fn test_cmp_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc9);
        cpu.memory.store(0x0101, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xd5);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xd5);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xcd);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xdd);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_absolute_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xd9);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_indirect_x() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc1);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x87);
        cpu.memory.store(0x0086, 0x00);
        cpu.memory.store(0x0087, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cmp_indirect_y() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0xff;
        cpu.registers.y = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xd1);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x86);
        cpu.memory.store(0x0085, 0x00);
        cpu.memory.store(0x0087, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    // CPX

    #[test]
    fn test_cpx_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe0);
        cpu.memory.store(0x0101, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cpx_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe4);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cpx_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xec);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    // CPY

    #[test]
    fn test_cpy_immediate() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc0);
        cpu.memory.store(0x0101, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cpy_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc4);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    #[test]
    fn test_cpy_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xff;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc4);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.get_flag(cpu::ZERO_FLAG));
    }

    // INC

    #[test]
    fn test_inc_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0xfe);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_inc_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xf6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0xfe);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    #[test]
    fn test_inc_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xee);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0xfe);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0xff);
    }

    #[test]
    fn test_inc_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xfe);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0xfe);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0xff);
    }

    // INX

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0xff; // -1
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xe8);

        assert!(cpu.registers.x == 0xff);
    }

    // INY

    #[test]
    fn test_iny() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0xfe; // -2
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc8);

        cpu.execute_instruction();

        assert!(cpu.registers.y == 0xff);
    }

    // DEC

    #[test]
    fn test_dec_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xc6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x01);
    }

    #[test]
    fn test_dec_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xd6);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x01);
    }

    #[test]
    fn test_dec_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xce);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x01);
    }

    #[test]
    fn test_dec_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xde);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x01);
    }

    // DEX

    #[test]
    fn test_dex() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x02;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xca);

        cpu.execute_instruction();

        assert!(cpu.registers.x == 0x01);
    }



    // DEY

    #[test]
    fn test_dey() {
        let mut cpu = Cpu::new();

        cpu.registers.y = 0x02;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x88);

        cpu.execute_instruction();

        assert!(cpu.registers.y == 0x01);
    }

    // ASL

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x2;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x0a);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x04);
    }

    #[test]
    fn test_asl_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x06);
        cpu.memory.store(0x0101, 0x084);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x04);
    }

    #[test]
    fn test_asl_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x16);
        cpu.memory.store(0x0101, 0x084);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x04);
    }

    #[test]
    fn test_asl_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x0e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x04);
    }

    #[test]
    fn test_asl_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x1e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x04);
    }

    // LSR

    #[test]
    fn test_lsr_accumulator() {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x2;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x4a);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x01);
    }

    #[test]
    fn test_lsr_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x46);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x01);
    }

    #[test]
    fn test_lsr_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x56);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x01);
    }

    #[test]
    fn test_lsr_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x4e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x01);
    }

    #[test]
    fn test_lsr_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 1;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x5e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x01);
    }

    // ROL

    #[test]
    fn test_rol_accumulator() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x2;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x2a);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x05);
    }

    #[test]
    fn test_rol_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x26);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x05);
    }

    #[test]
    fn test_rol_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.x = 0x01;
        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x36);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x05);
    }

    #[test]
    fn test_rol_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x2e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x05);
    }

    #[test]
    fn test_rol_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x3e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x02);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x05);
    }

    // ROR

    #[test]
    fn test_ror_accumulator() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.accumulator = 0x08;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x6a);

        cpu.execute_instruction();

        assert!(cpu.registers.accumulator == 0x84);
    }

    #[test]
    fn test_ror_zero_page() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status = cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x66);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0084, 0x08);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x84);
    }

    #[test]
    fn test_ror_zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x76);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0085, 0x08);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x84);
    }

    #[test]
    fn test_ror_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x6e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0084, 0x08);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0084) == 0x84);
    }

    #[test]
    fn test_ror_absolute_x() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.x = 0x01;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x7e);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x00);
        cpu.memory.store(0x0085, 0x08);

        cpu.execute_instruction();

        assert!(cpu.memory.fetch(0x0085) == 0x84);
    }

    // JMP

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x4c);
        cpu.memory.store(0x0101, 0xff);
        cpu.memory.store(0x0102, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x01ff);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x6c);
        cpu.memory.store(0x0101, 0x84);
        cpu.memory.store(0x0102, 0x01);
        cpu.memory.store(0x0184, 0xff);
        cpu.memory.store(0x0185, 0xff);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0xffff);
    }

    // JSR

    #[test]
    fn test_jsr() {
        let mut cpu = Cpu::new();

        cpu.reset();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x20);
        cpu.memory.store(0x0101, 0xff);
        cpu.memory.store(0x0102, 0x01);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x01ff);

        assert!(cpu.memory.fetch(0x01fd) == 0x01);

        assert!(cpu.memory.fetch(0x01fc) == 0x02);
    }

    // RTS

    #[test]
    fn test_rts() {
        let mut cpu = Cpu::new();

        cpu.reset();

        cpu.registers.program_counter = 0x0100;
        cpu.push_word(0x0102);

        cpu.memory.store(0x0100, 0x60);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0103);
    }

    // BCC

    #[test]
    fn test_bcc() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x90);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0102);
    }

    // BCS

    #[test]
    fn test_bcs() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::CARRY_BIT;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xb0);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }

    // BEQ

    #[test]
    fn test_beq() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::ZERO_FLAG;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xf0);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }

    // BMI

    #[test]
    fn test_bmi() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::NEGATIVE_FLAG;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x30);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }

    // BNE

    #[test]
    fn test_bne() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0xd0);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }

    // BPL

    #[test]
    fn test_bpl() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x10);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }

    // BVC

    #[test]
    fn test_bvc() {
        let mut cpu = Cpu::new();

        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x50);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }

    // BVS

    #[test]
    fn test_bvs() {
        let mut cpu = Cpu::new();

        cpu.registers.processor_status |= cpu::OVERFLOW_FLAG;
        cpu.registers.program_counter = 0x0100;

        cpu.memory.store(0x0100, 0x70);
        cpu.memory.store(0x0101, 0x02);

        cpu.execute_instruction();

        assert!(cpu.registers.program_counter == 0x0104);
    }
}