struct Cpu {
    registers: [u8; 0xF],
    current_operation: u16,
}

impl Cpu {
    fn run(&mut self) {
        let hh = (self.current_operation & 0xF000) >> 12;
        let hl = (self.current_operation & 0x0F00) >> 8;
        let lh = (self.current_operation & 0x00F0) >> 4;
        let ll = self.current_operation & 0x000F;

        match (hh, hl, lh, ll) {
            (8, _, _, 4) => {
                self.registers[hl as usize] = self.registers[hl as usize] + self.registers[lh as usize];
            },
            _ => {
                
            }
        }
    }
}

fn main() {
    let mut cpu = Cpu {
        registers: [0; 0xF],
        current_operation: 0,
    };

    cpu.registers[0] = 1;
    cpu.registers[1] = 3;
    cpu.current_operation = 0x8014;

    cpu.run();
    println!("{}", cpu.registers[0]);
}
