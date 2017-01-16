mod cpu;

fn main() {
    let mut cpu = cpu::cpu::Cpu::new();
    cpu.run();
    println!("{:?}", cpu);
}
