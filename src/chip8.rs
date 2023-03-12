use std::time::{Instant, Duration};

use rand::Rng;

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
    pub waiting_keypress: (bool, u16),
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub elapsed_time: Duration,
    pub draw: bool,
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
            waiting_keypress: (false, 0),
            delay_timer: 0,
            sound_timer: 0,
            elapsed_time: Duration::from_secs(0),
            draw: true,
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
    
    pub fn key_down(&mut self, key: u8) {
        println!("KEY DOWN: {:#01x}", key);
        self.keys[key as usize] = 1;

        if self.waiting_keypress.0 {
            self.registers[self.waiting_keypress.1 as usize] = key;
            self.waiting_keypress = (false, 0);
        }
    }

    pub fn key_up(&mut self, key: u8) {
        println!("KEY UP: {:#01x}", key);
        self.keys[key as usize] = 0;
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
        self.registers[x as usize] &= self.registers[y as usize];
    }

    pub fn xor(&mut self, x: u16, y: u16) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    pub fn add_xy(&mut self, x: u16, y: u16) {
        let v_x = self.registers[x as usize];
        let v_y = self.registers[y as usize];
        self.registers[0xF] = 0;

        let (result, overflow) = v_x.overflowing_add(v_y);
        if overflow {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
    }

    pub fn sub_xy(&mut self, x: u16, y: u16) {
        let v_x = self.registers[x as usize];
        let v_y = self.registers[y as usize];
        self.registers[0xF] = 0;

        let (result, _) = v_x.overflowing_sub(v_y);
        if v_x > v_y {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
    }

    pub fn subn_xy(&mut self, x: u16, y: u16) {
        let v_x = self.registers[x as usize];
        let v_y = self.registers[y as usize];
        self.registers[0xF] = 0;

        let (result, _) = v_y.overflowing_sub(v_x);
        if v_y > v_x {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
    }

    pub fn add_xkk(&mut self, x: u16, byte: u8) {
        let v_x = self.registers[x as usize];
        self.registers[0xF] = 0;

        let (result, overflow) = v_x.overflowing_add(byte as u8);
        if overflow {
            self.registers[0xF] = 1;
        }
        
        self.registers[x as usize] = result as u8;
    }

    pub fn shr_x(&mut self, x: u16) {
        println!("Set V{} = V{} SHR 1.", x, x);
        let v_x = self.registers[x as usize];
        
        self.registers[x as usize] = self.registers[x as usize] >> 1;
        self.registers[0xF] = v_x & 0b1;
    }
    
    pub fn shl_x(&mut self, x: u16) {
        println!("Set V{} = V{} SHL 1.", x, x);
        let v_x = self.registers[x as usize];
        
        self.registers[x as usize] = self.registers[x as usize] << 1;
        self.registers[0xF] = (v_x >> 7) & 0b1;
    }

    pub fn call(&mut self, addr: u16) {
        println!("Call subroutine at {:#x}.", addr);
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
        println!("Return to {:#x}", self.stack[sp-1]);

        if sp <= 0 {
            panic!("Stack underflow error");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[sp-1] as usize;
    }

    pub fn skp_vx(&mut self, x: u16) {
        let v_x = self.registers[x as usize];

        if self.keys[v_x as usize] == 1 {
            self.program_counter += 2;
        }
    }

    pub fn sknp_vx(&mut self, x: u16) {
        let v_x = self.registers[x as usize];

        if self.keys[v_x as usize] == 0 {
            self.program_counter += 2;
        }
    }

    pub fn ld_vx_dt(&mut self, x: u16) {
        self.registers[x as usize] = self.delay_timer;
    }

    pub fn ld_vx_k(&mut self, x: u16) {
        self.waiting_keypress = (true, x);
    }

    pub fn ld_dt_vx(&mut self, x: u16) {
        self.delay_timer = self.registers[x as usize];
    }

    pub fn ld_st_vx(&mut self, x: u16) {
        self.sound_timer = self.registers[x as usize];
    }

    pub fn add_i_vx(&mut self, x: u16) {
        println!("Set I = I + V{}.", x);

        self.regI += self.registers[x as usize] as u16;
    }
    
    pub fn ld_b_vx(&mut self, x: u16) {
        println!("Store BCD representation of V{} in memory locations I, I+1, and I+2.", x);
        let mut v_x = self.registers[x as usize];

        self.memory[self.regI as usize] = v_x / 100;
        v_x %= 100;
        self.memory[(self.regI + 1) as usize] = v_x / 10;
        v_x %= 10;
        self.memory[(self.regI + 2) as usize] = v_x;
    }

    pub fn rnd(&mut self, x: u16, byte: u8) {
        println!("Set V{} = random byte AND {:#x}.", x, byte);

        let rand: u8 = rand::thread_rng().gen_range(0..=255);
        self.registers[x as usize] = rand & byte;
    }

    pub fn ld_i_vx(&mut self, x: u16) {
        println!("Store registers V0 through V{} in  memory starting at location {:#x}.", x, self.regI);

        self.memory[self.regI as usize..(self.regI + x + 1) as usize]
            .copy_from_slice(&self.registers[0..(x + 1) as usize]);
    }

    pub fn ld_f_vx(&mut self, x: u16) {
        let mut v_x = self.registers[x as usize];

        self.regI = v_x as u16 * 5;
    }

    pub fn ld_vx_i(&mut self, x: u16) {
        println!("Read registers V0 through V{} from memory starting at location {:#x}.", x, self.regI);

        self.registers[0..(x + 1) as usize]
            .copy_from_slice(&self.memory[self.regI as usize..(self.regI + x + 1) as usize]);
    }

    pub fn exec(&mut self, opcode: u16) {
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
                return;
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
                self.or(hl, y);
            },
            (8, _, _, 2) => {
                self.and(x, y);
            },
            (8, _, _, 3) => {
                self.xor(x, y);
            },
            (8, _, _, 4) => {
                self.add_xy(x, y);
            },
            (8, _, _, 5) => {
                self.sub_xy(x, y);
            },
            (8, _, _, 6) => {
                self.shr_x(x);
            },
            (8, _, _, 7) => {
                self.subn_xy(x, y);
            }
            (8, _, _, 0xE) => {
                self.shl_x(x);
            }
            (9, _, _, 0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += 2
                }
            }
            (0xA, _, _, _) => {
                self.regI = addr;
            },
            (0xB, _, _, _) => {
                self.program_counter = (self.registers[0] + addr as u8) as usize;
            },
            (0xC, _, _, _) => {
                self.rnd(x, byte);
            }
            (0xD, _, _, _) => {
                let x = self.registers[hl as usize];
                let y = self.registers[lh as usize];
                let height = nibble;
                
                self.registers[0xF] = 0;
                for byte_n in 0..height {
                    let current_byte = self.memory[(self.regI + byte_n) as usize];

                    for i in 0..8 {
                        let bit = &mut self.display[((y + byte_n as u8) % 32) as usize][((x + i) % 64) as usize];
                        if (*bit) == 1 && (((*bit) ^ (current_byte >> (7 - i)) & 0b1) == 0) {
                            self.registers[0xF] = 1;
                        }
                        (*bit) ^= (current_byte >> (7 - i)) & 0b1;
                    }
                }
                self.draw = true;
            },
            (0xE, _, 9, 0xE) => {
                self.skp_vx(x);
            },
            (0xE, _, 0xA, 1) => {
                self.sknp_vx(x);
            },
            (0xF, _, 0, 7) => {
                self.ld_vx_dt(x);
            },
            (0xF, _, 0, 0xA) => {
                self.ld_vx_k(x);
            },
            (0xF, _, 1, 5) => {
                self.ld_dt_vx(x);
            },
            (0xF, _, 1, 8) => {
                self.ld_st_vx(x);
            },
            (0xF, _, 1, 0xE) => {
                self.add_i_vx(x);
            },
            (0xF, _, 2, 9) => {
                self.ld_f_vx(x);
            }
            (0xF, _, 3, 3) => {
                self.ld_b_vx(x);
            }
            (0xF, _, 5, 5) => {
                self.ld_i_vx(x);
            },
            (0xF, _, 6, 5) => {
                self.ld_vx_i(x);
            }
            _ => {}
        }

        self.program_counter += 2;
    }

    pub fn cycle(&mut self) {
        if !self.waiting_keypress.0 {
            let opcode = self.read_opcode();
            println!("Addr: {:#04x} | Opcode: {:#04x}", self.program_counter, opcode);
            
            self.exec(opcode);
        }
        // println!("ELAPSED: {:?}", self.elapsed_time.as_secs_f32());
        // println!("1/60s = {:?}", Duration::from_secs_f32(1.0/60.0).as_secs_f32());
        // println!("delay timer: {}", self.delay_timer);

        if self.elapsed_time >= Duration::from_secs_f32(1.0/60.0) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            self.elapsed_time = Duration::from_secs(0);
        }
    }

}