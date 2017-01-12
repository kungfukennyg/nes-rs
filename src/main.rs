mod cpu;

fn main() {
    let cpu = cpu::cpu::Cpu::new();

    println!("{:?}", cpu);
}
