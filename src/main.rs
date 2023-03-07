struct Cpu {
    registers: [u8; 0xF],
    current_operation: u16,
}

fn main() {
    let mut cpu = Cpu {
        registers: [0; 0xF],
        current_operation: 0,
    };

    cpu.registers[0] = 1;
    cpu.registers[1] = 3;
    cpu.current_operation = 0x8014;

    println!("{}", cpu.registers[0]);
}
