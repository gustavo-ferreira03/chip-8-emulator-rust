static fonts: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    pub registers: [u8; 16],
    pub regI: u16,
    pub program_counter: usize,
    pub memory: [u8; 0x1000],
    pub stack_pointer: usize,
    pub stack: [u16; 16],
    pub display: [[u8; 64]; 32],
    pub keys: [u8; 16],
    pub time: isize,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            registers: [0; 16],
            regI: 0,
            program_counter: 0x200,
            memory: [0; 0x1000],
            stack_pointer: 0,
            stack: [0; 16],
            display: [[0; 64]; 32],
            keys: [0; 16],
            time: 0,
        };
        chip8.memory[0..80].copy_from_slice(&fonts);
        chip8
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
        let v_x = self.registers[x as usize];
        let v_y = self.registers[y as usize];

        let (result, overflow) = v_x.overflowing_add(v_y);
        if overflow {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
    }

    pub fn sub_xy(&mut self, x: u16, y: u16) {
        let v_x = self.registers[x as usize];
        let v_y = self.registers[y as usize];

        let (result, overflow) = v_x.overflowing_sub(v_y);
        if overflow {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
    }

    pub fn add_xkk(&mut self, x: u16, byte: u8) {
        let v_x = self.registers[x as usize];

        let (result, overflow) = v_x.overflowing_add(byte as u8);
        if overflow {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
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

    pub fn skp(&mut self, x: u16) {
        let v_x = self.registers[x as usize];

        if self.keys[v_x as usize] == 1 {
            self.program_counter += 2;
        }
    }

    pub fn sknp(&mut self, x: u16) {
        let v_x = self.registers[x as usize];

        if self.keys[v_x as usize] == 0 {
            self.program_counter += 2;
        }
    }

    pub fn key_down(&mut self, key: u16) {
        println!("KEY DOWN: {:#01x}", key);
        self.keys[key as usize] = 1;
    }

    pub fn key_up(&mut self, key: u16) {
        println!("KEY UP: {:#01x}", key);
        self.keys[key as usize] = 0;
    }

    pub fn run(&mut self, opcode: u16) {
        let hh = (opcode & 0xF000) >> 12;
        let hl = (opcode & 0x0F00) >> 8;
        let lh = (opcode & 0x00F0) >> 4;
        let ll = (opcode & 0x000F) >> 0;

        let x = hl;
        let y = lh;
        let addr = (hl << 8) | (lh << 4) | (ll << 0);
        let byte = ((lh << 4) | (ll << 0)) as u8;
        let nibble = ll;

        match (hh, hl, lh, ll) {
            (0, 0, 0xE, 0x0) => {
                self.display = [[0; 64]; 32];
            }
            (0, 0, 0xE, 0xE) => {
                self.ret();
            },
            (1, _, _, _) => {
                // println!(" - JUMP {:#04x}", addr);
                self.program_counter = addr as usize;
                return;
            },
            (2, _, _, _) => {
                self.call(addr);
            },
            (3, _, _, _) => {
                if self.equal_xkk(x, byte) {
                    self.program_counter += 2;
                }
            },
            (4, _, _, _) => {
                if !self.equal_xkk(x, byte) {
                    self.program_counter += 2;
                }
            },
            (5, _, _, 0) => {
                if self.equal_xy(x, y) {
                    self.program_counter += 2;
                }
            },
            (6, _, _, _) => {
                self.registers[hl as usize] = byte;
            },
            (7, _, _, _) => {
                self.add_xkk(x, byte);
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
            (9, _, _, 0) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += 2
                }
            }
            (0xA, _, _, _) => {
                self.regI = addr;
            },
            (0xB, _, _, _) => {
                self.program_counter = (self.registers[0] + addr as u8) as usize;
            },
            (0xD, _, _, _) => {
                let x = self.registers[hl as usize];
                let y = self.registers[lh as usize];
                let height = nibble;

                // println!("HEIGHT: {}", height);
                if x < 56 && y < (32 - height) as u8 {
                    for byte_n in 0..height {
                        let current_byte = self.memory[(self.regI + byte_n) as usize];
                        // println!("CURRENT BYTE: {:#08b}", current_byte);

                        for (i, bit) in self.display[(y + byte_n as u8) as usize][x as usize..(x + 8) as usize].iter_mut().enumerate() {
                            if (*bit) ^ (current_byte >> (7 - i)) & 0b1 == 1 {
                                self.registers[0xF] = 1;
                            }
                            (*bit) ^= (current_byte >> (7 - i)) & 0b1;
                        }
                    }
                }
            },
            (0xE, _, 9, 0xE) => {
                self.skp(x);
            },
            (0xE, _, 0xA, 1) => {
                self.sknp(x);
            }
            _ => {}
        }

        self.program_counter += 2;
    }

    pub fn cycle(&mut self) {
        let opcode = self.read_opcode();
        println!("Addr: {:#04x} | Opcode: {:#04x}", self.program_counter, opcode);
        
        self.run(opcode);
    }

}