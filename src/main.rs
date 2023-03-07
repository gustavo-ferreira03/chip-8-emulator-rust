struct Cpu {
    registers: [u8; 0xF],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack_pointer: usize,
    stack: [u16; 0xF],
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn add_registers(&mut self, reg1: u16, reg2: u16) {
        self.registers[reg1 as usize] += self.registers[reg2 as usize];
    }

    fn call(&mut self, mem_address: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow error");
        }
        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = mem_address as usize;
    }

    fn ret(&mut self) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp <= 0 {
            panic!("Stack underflow error");
        }
        self.stack_pointer -= 1;
        self.program_counter = stack[sp] as usize;
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            if opcode == 0 {
                break;
            }
            
            let hh = (opcode & 0xF000) >> 12;
            let hl = (opcode & 0x0F00) >> 8;
            let lh = (opcode & 0x00F0) >> 4;
            let ll = (opcode & 0x000F) >> 0;

            match (hh, hl, lh, ll) {
                (2, _, _, _) => {
                    let mem_address = (hl << 8) | (lh << 4) | (ll << 0);
                    self.call(mem_address);
                },
                (0, 0, 0xE, 0xE) => {
                    self.ret();
                }
                (8, _, _, 4) => {
                    self.add_registers(hl, lh);
                },
                _ => {}
            }
            
            self.program_counter += 2;
        }
    }
}

fn main() {
    let mut cpu = Cpu {
        registers: [0; 0xF],
        program_counter: 0,
        memory: [0; 0x1000],
        stack_pointer: 0,
        stack: [0; 0xF],
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0] = 0x21;
    cpu.memory[1] = 0x00;

    let add_twice: [u8; 6] = [
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE
    ];
    cpu.memory[0x100..0x106].copy_from_slice(&add_twice);

    cpu.run();
    println!("{}", cpu.registers[0]);
}
