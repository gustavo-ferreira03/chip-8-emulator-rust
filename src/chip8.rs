pub struct Chip8 {
    pub registers: [u8; 0xF],
    pub regI: u16,
    pub program_counter: usize,
    pub memory: [u8; 0x1000],
    pub stack_pointer: usize,
    pub stack: [u16; 0xF],
    pub display: [[u8; 32]; 64],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            registers: [0; 0xF],
            regI: 0,
            program_counter: 0,
            memory: [0; 0x1000],
            stack_pointer: 0,
            stack: [0; 0xF],
            display: [[0; 32]; 64]
        }
    }

    pub fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }
    
    pub fn get_addr(&self, n1: u16, n2: u16, n3: u16) -> u16 {
        (n1 << 8) | (n2 << 4) | (n3 << 0)
    }

    pub fn get_byte(&self, k1: u16, k2: u16) -> u8 {
        ((k1 << 4) | (k2 << 0)) as u8
    }

    pub fn equal_xkk(&self, x: u16, byte: u8) -> bool {
        self.registers[x as usize] == byte
    }

    pub fn equal_xy(&self, x: u16, y: u16) -> bool {
        self.registers[x as usize] ==  self.registers[y as usize]
    }

    pub fn or(&mut self, x: u16, y: u16) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    pub fn and(&mut self, x: u16, y: u16) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    pub fn xor(&mut self, x: u16, y: u16) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    pub fn add_xy(&mut self, x: u16, y: u16) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    pub fn sub_xy(&mut self, x: u16, y: u16) {
        self.registers[x as usize] -= self.registers[y as usize];
    }

    pub fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow error");
        }
        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    pub fn ret(&mut self) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp <= 0 {
            panic!("Stack underflow error");
        }
        self.stack_pointer -= 1;
        self.program_counter = stack[sp] as usize;
    }

    pub fn run(&mut self) {
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
                (0, 0, 0xE, 0x0) => {
                    self.display = [[0; 32]; 64];
                }
                (0, 0, 0xE, 0xE) => {
                    self.ret();
                },
                (1, _, _, _) => {
                    let addr = self.get_addr(hl, lh, ll);
                    self.program_counter = addr as usize;
                },
                (2, _, _, _) => {
                    let addr = self.get_addr(hl, lh, ll);
                    self.call(addr);
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
                },
                (6, _, _, _) => {
                    let byte = self.get_byte(lh, ll);
                    self.registers[hl as usize] = byte;
                },
                (7, _, _, _) => {
                    let byte = self.get_byte(lh, ll);
                    self.registers[hl as usize] += byte;
                },
                (8, _, _, 0) => {
                    self.registers[hl as usize] = self.registers[lh as usize];
                },
                (8, _, _, 1) => {
                    self.or(hl, lh);
                },
                (8, _, _, 2) => {
                    self.and(hl, lh);
                },
                (8, _, _, 3) => {
                    self.xor(hl, lh);
                },
                (8, _, _, 4) => {
                    self.add_xy(hl, lh);
                },
                (8, _, _, 5) => {
                    self.sub_xy(hl, lh);
                },
                (0xA, _, _, _) => {
                    let addr = self.get_addr(hl, lh, ll);
                    self.regI = addr;
                },
                (0xD, _, _, _) => {
                    let x = self.registers[hl as usize];
                    let y = self.registers[lh as usize];
                    let n_bytes = ll;

                    if x < 56 {
                        for byte_n in 0..n_bytes {
                            let current_byte = self.memory[(self.regI + byte_n * 2) as usize];

                            for (i, bit) in &mut self.display[y as usize].iter_mut().enumerate() {
                                if *bit ^ (current_byte >> (7 - i)) == 1 {
                                    self.registers[0xF] = 1;
                                }
                                *bit ^= current_byte >> (7 - i);
                            }
                        }
                    }
                }
                _ => {}
            }

            self.program_counter += 2;
        }
    }
}