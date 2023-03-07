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
    
    fn get_addr(&self, n1: u16, n2: u16, n3: u16) -> u16 {
        (n1 << 8) | (n2 << 4) | (n3 << 0)
    }

    fn get_byte(&self, k1: u16, k2: u16) -> u8 {
        ((k1 << 4) | (k2 << 0)) as u8
    }

    fn equal_xkk(&self, x: u16, byte: u8) -> bool {
        self.registers[x as usize] == byte
    }

    fn equal_xy(&self, x: u16, y: u16) -> bool {
        self.registers[x as usize] ==  self.registers[y as usize]
    }

    fn add_xy(&mut self, x: u16, y: u16) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    fn sub_xy(&mut self, x: u16, y: u16) {
        self.registers[x as usize] -= self.registers[y as usize];
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
                (0, 0, 0xE, 0xE) => {
                    self.ret();
                },
                (1, _, _, _) => {
                    let mem_address = self.get_addr(hl, lh, ll);
                    self.program_counter = mem_address as usize;
                },
                (2, _, _, _) => {
                    let mem_address = self.get_addr(hl, lh, ll);
                    self.call(mem_address);
                },
                (3, _, _, _) => {
                    let byte = self.get_byte(lh, ll);
                    if self.equal_xkk(hl, byte) {
                        self.program_counter += 2;
                    }
                },
                (4, _, _, _) => {
                    let byte = self.get_byte(lh, ll);
                    if !self.equal_xkk(hl, byte) {
                        self.program_counter += 2;
                    }
                },
                (5, _, _, 0) => {
                    if self.equal_xy(lh, hl) {
                        self.program_counter += 2;
                    }
                }
                (8, _, _, 4) => {
                    self.add_xy(hl, lh);
                },
                (8, _, _, 5) => {
                    self.sub_xy(hl, lh);
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

    let add_twice: [u8; 12] = [
        0x80, 0x14,
        0x80, 0x14,
        0x80, 0x14,
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE
    ];
    cpu.memory[0x100..0x10C].copy_from_slice(&add_twice);

    cpu.run();
    println!("{}", cpu.registers[0]);
}
