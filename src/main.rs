mod nes;

use nes::cpu::CPu;

fn main() {
    let mut cpu = Cpu::new();
    println!("{:?}", cpu);
}
