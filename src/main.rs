struct Cpu {
    registers: [u8; 0xF],
    program_counter: usize,
    memory: [u8; 0x1000],
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            if opcode == 0 {
                break;
            }
            self.program_counter += 2;
            
            let hh = (opcode & 0xF000) >> 12;
            let hl = (opcode & 0x0F00) >> 8;
            let lh = (opcode & 0x00F0) >> 4;
            let ll = (opcode & 0x000F) >> 0;

            match (hh, hl, lh, ll) {
                (8, _, _, 4) => {
                    self.registers[hl as usize] +=  self.registers[lh as usize];
                },
                _ => {
                    
                }
            }
        }
    }
}

fn main() {
    let mut cpu = Cpu {
        registers: [0; 0xF],
        program_counter: 0,
        memory: [0; 0x1000],
    };
    cpu.registers[0] = 1;
    cpu.registers[1] = 3;

    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x14;
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x14;

    cpu.run();
    println!("{}", cpu.registers[0]);
    
    assert_eq!(cpu.registers[0], 10);
}
