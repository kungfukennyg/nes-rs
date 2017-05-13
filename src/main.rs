#![allow(dead_code, unused_assignments, unused_mut)]
mod nes;
mod tests;

use nes::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    println!("{:?}", cpu);
}
