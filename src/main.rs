extern crate time;
extern crate core;

#[allow(dead_code, unused_assignments, unused_mut)]
mod nes;
mod tests;

use nes::cpu::Cpu;
use nes::rom::Rom;

use std::env;
use std::path::Path;
use std::fs::File;
use std::boxed::Box;


/// TODO:
/// - Test CPU
/// - Implement PPU
/// - Implement keyboard input
/// - Implement APU
///
fn main() {
    let rom_path = env::args().skip(1).next().expect("Failed to find rom file");
    let rom = Box::new(Rom::load(&mut File::open(&Path::new(&rom_path)).unwrap()));

    println!("Rom loaded: {}", rom.header);

    let mut cpu = Cpu::new(rom);
    println!("{:?}", cpu);

    cpu.reset();

    let mut last_time = time::precise_time_s();
    let mut frame = 0;

    loop {
        cpu.execute_instruction();
    }
}
