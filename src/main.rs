#[macro_use]
extern crate time;

#[allow(dead_code, unused_assignments, unused_mut)]
mod nes;
mod tests;

use nes::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    println!("{:?}", cpu);

    cpu.reset();

    let mut last_time = time::precise_time_s();
    let mut frame = 0;

    loop {
        cpu.execute_instruction();
    }
}
