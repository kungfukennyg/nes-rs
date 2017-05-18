use super::rom::Rom;

#[derive(PartialEq, Eq)]
pub enum MapperResult {
    Continue,
    Irq,
}

pub trait Mapper {
    fn prg_load(&mut self, addr: u16) -> u8;
    fn prg_store(&mut self, addr: u16, val: u8);
    fn chr_load(&mut self, addr: u16) -> u8;
    fn chr_store(&mut self, addr: u16, val: u8);
    fn next_scanline(&mut self) -> MapperResult;
}


/// Mapper 0 (NROM)
///
/// See: http://wiki.nesdev.com/w/index.php/NROM
pub struct Nrom {
    pub rom: Box<Rom>,
}

impl Mapper for Nrom {
    fn prg_load(&mut self, addr: u16) -> u8 {
        if addr < 0x8000 {
            0u8
        } else if self.rom.prg.len() > 16384 {
            self.rom.prg[addr as usize & 0x7fff]
        } else {
            self.rom.prg[addr as usize & 0x3fff]
        }
    }
    /// can't write to PRG-ROM
    fn prg_store(&mut self, addr: u16, val: u8) {
        panic!("Unsupported");
    }
    fn chr_load(&mut self, addr: u16) -> u8 {
        self.rom.chr[addr as usize]
    }
    /// can't write to CHR-ROM
    fn chr_store(&mut self, addr: u16, val: u8) {
        panic!("Unsupported");
    }
    fn next_scanline(&mut self) -> MapperResult {
        MapperResult::Continue
    }
}

/// Mapper 1 (SxROM/MMC1)
///
/// See: http://wiki.nesdev.com/w/index.php/Nintendo_MMC1
#[derive(Copy, Clone)]
struct SxCtrl {
    val: u8
}

pub enum Mirroring {
    OneScreenLower,
    OneScreenUpper,
    Vertical,
    Horizontal,
}

enum SxPrgBankMode {
    /// Switch 32K at $8000, ignore lower bit
    Switch32k,
    /// Fix first bank at $8000, switch 16K bank at $C000
    FixFirstBank,
    /// Fix last bank at $C000, switch 16K bank at $8000
    FixLastBank,
}

impl SxCtrl {
    fn prg_rom_mode(&self) -> SxPrgBankMode {
        match (self.val >> 2) & 3 {
            0 | 1 => SxPrgBankMode::Switch32k,
            2 => SxPrgBankMode::FixFirstBank,
            3 => SxPrgBankMode::FixLastBank,
            _ => panic!("can't happen")
        }
    }
}

#[derive(Copy, Clone)]
struct SxRegs {
    /// $8000-$9FFF
    ctrl: SxCtrl,
    /// $A000-$BFFF
    chr_bank_0: u8,
    /// $C000-$DFFF
    chr_bank_1: u8,
    /// #E000-$FFFF
    prg_bank: u8
}

pub struct SxRom {
    rom: Box<Rom>,
    regs: SxRegs,
    /// internal accumulator
    accum: u8,
    /// write count (at 5 update register)
    write_count: u8,
    chr_ram: Box<[u8; 8192]>,
}

impl SxRom {
    fn new(rom: Box<Rom>) -> SxRom {
        SxRom {
            rom: rom,
            regs: SxRegs {
                ctrl: SxCtrl {
                    val: 3 << 2,
                },
                chr_bank_0: 0,
                chr_bank_1: 0,
                prg_bank: 0
            },
            accum: 0,
            write_count: 0,
            chr_ram: Box::new([0; 8192]),
        }
    }
}

impl Mapper for SxRom {
    fn prg_load(&mut self, addr: u16) -> u8 {
        if addr < 0x8000 {
            0u8
        } else if addr < 0xc000 {
            let bank = match self.regs.ctrl.prg_rom_mode() {
                SxPrgBankMode::Switch32k => self.regs.prg_bank & 0xfe,
                SxPrgBankMode::FixFirstBank => 0,
                SxPrgBankMode::FixLastBank => self.regs.prg_bank,
            };

            self.rom.prg[(bank as usize * 16384) | ((addr & 0x3fff) as usize)]
        } else {
            let bank = match self.regs.ctrl.prg_rom_mode() {
                SxPrgBankMode::Switch32k => (self.regs.prg_bank & 0xfe) | 1,
                SxPrgBankMode::FixFirstBank => self.regs.prg_bank,
                SxPrgBankMode::FixLastBank => (*self.rom).header.prg_rom_size - 1,
            };
            self.rom.prg[(bank as usize * 16384) | ((addr & 0x3fff) as usize)]
        }
    }

    fn prg_store(&mut self, addr: u16, val: u8) {
        if addr < 0x8000 {
            return;
        }

        // check reset
        if (val & 0x80) != 0 {
            self.write_count = 0;
            self.accum = 0;
            self.regs.ctrl = SxCtrl {
                val: self.regs.ctrl.val | (3 << 2)
            };
            return;
        }

        // write lowest bit of value into accumulator
        self.accum |= (val & 1) << (self.write_count as usize);

        self.write_count += 1;
        if self.write_count == 5 {
            self.write_count = 0;

            // write to internal register
            if addr <= 0x9fff {
                self.regs.ctrl = SxCtrl {
                    val: self.accum
                };
            } else if addr <= 0xbfff {
                self.regs.chr_bank_0 = self.accum;
            } else if addr <= 0xdfff {
                self.regs.chr_bank_1 = self.accum;
            } else {
                self.regs.prg_bank = self.accum;
            }

            self.accum = 0;
        }
    }

    fn chr_load(&mut self, addr: u16) -> u8 {
        self.chr_ram[addr as usize]
    }

    fn chr_store(&mut self, addr: u16, val: u8) {
        self.chr_ram[addr as usize] = val;
    }

    fn next_scanline(&mut self) -> MapperResult {
        MapperResult::Continue
    }
}