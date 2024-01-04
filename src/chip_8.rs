use rand::Rng;
use registers::Registers;
use stack::Stack;

mod registers;
mod stack;

pub struct Chip8 {
    ram: [u8; 4096],
    regs: Registers,
    stack: Stack,
    pub is_halt: bool,
}

impl Chip8 {

    pub fn new(rom: &Vec<u8>) -> Self {
        let offset = 0x200;

        let mut this = Chip8::new_internal();

        // Initialize RAM with ROM
        Chip8::load_sprites(&mut this.ram);
        for (i, value) in rom.iter().enumerate() {
            this.ram[i + offset] = dbg!(*value);
        }

        this.regs.PC = 512;

        this
    }

    pub fn emulate_cycle(&mut self) {
        let instruction: u16 = ((self.ram[usize::from(self.regs.PC)] as u16) << 8) | self.ram[usize::from(self.regs.PC + 1)] as u16;
        let nnn: u16= instruction & 0x0FFF;
        let n: u8 = instruction as u8 & 0x0F;
        let x: u8 = (instruction >> 8) as u8 & 0x0F;
        let y: u8 = instruction as u8 & 0xF0;
        let kk: u8 = instruction as u8;

        match instruction & 0xF000 {
            0x0000 => {
                match instruction & 0x00FF {
                    // CLS
                    0x00E0 => {}
                    // RET
                    0x00EE => {
                        let address: u16 = self.stack.pop();
                        self.regs.PC = address;
                    }
                    _ =>  panic!("Unexceptected instructions {}", instruction),
                }
            }
            // JP addr
            0x1000 => {
                self.regs.PC = nnn;
            },
            // CALL addr
            0x2000 => {
                self.stack.push(self.regs.PC);
            },
            // SE Vx, byte
            0x3000 => {
                if self.regs.V[usize::from(x)] == kk {
                    self.regs.PC += 2;
                }
            },
            // SNE Vx, byte
            0x4000 => {
                if self.regs.V[usize::from(x)] != kk {
                    self.regs.PC += 2;
                }
            }
            // SE Vx, Vy
            0x5000 => {
                if self.regs.V[usize::from(x)] == self.regs.V[usize::from(y)] {
                    self.regs.PC += 2;
                }
            }
            // LD Vx, byte
            0x6000 => { 
                self.regs.V[usize::from(x)] = kk; 
            }
            // ADD Vx, byte
            0x7000 => { 
                self.regs.V[usize::from(x)] = self.regs.V[usize::from(x)] + kk; 
            }
            0x8000 => match instruction & 0x000F {
                // LD Vx, Vy
                0x0000 => self.regs.V[usize::from(x)] = self.regs.V[usize::from(y)],
                // OR Vx, Vy
                0x0001 => self.regs.V[usize::from(x)] |= self.regs.V[usize::from(y)],
                // AND Vx, Vy
                0x0002 => self.regs.V[usize::from(x)] &= self.regs.V[usize::from(y)],
                // XOR Vx, Vy
                0x0003 => self.regs.V[usize::from(x)] ^= self.regs.V[usize::from(y)],
                // ADD Vx, Vy
                0x0004 => {
                    let x: u8 = self.regs.V[usize::from(x)];

                    self.regs.V[usize::from(x)] = x.wrapping_add(self.regs.V[usize::from(y)]);
                    self.regs.V[0xF] = if x < self.regs.V[usize::from(x)] { 1 } else { 0 };
                }
                // SUB Vx, Vy
                0x0005 => {
                    let x: u8 = self.regs.V[usize::from(x)];

                    self.regs.V[usize::from(x)] = x.wrapping_sub(self.regs.V[usize::from(y)]);
                    self.regs.V[0xF] = if x > self.regs.V[usize::from(x)] { 1 } else { 0 };
                }
                // SHR Vx {, Vy}
                0x0006 => {
                    let x: u8 = self.regs.V[usize::from(x)].wrapping_shr(1);
                    let overflow_flag: u8 = self.regs.V[usize::from(x)] & 0x1;

                    self.regs.V[0xF] = overflow_flag;
                    self.regs.V[usize::from(x)] = x;
                }
                // SUBN Vx, Vy
                0x0007 => {
                    let not_borrow: u8 = (self.regs.V[usize::from(y)] > self.regs.V[usize::from(x)]) as u8;

                    self.regs.V[usize::from(x)] = self.regs.V[usize::from(y)] - self.regs.V[usize::from(x)];
                    self.regs.V[0xF] = not_borrow;
                }
                // SHL Vx {, Vy}
                0x000E => {
                    let x: u8 = self.regs.V[usize::from(x)].wrapping_shl(1);
                    let overflow_flag: u8 = self.regs.V[usize::from(x)] & 0x80;

                    self.regs.V[0xF] = overflow_flag;
                    self.regs.V[usize::from(x)] = x;
                }
                _ => panic!("Unexpected instruction {}", instruction),
            }
            // SNE Vx, Vy
            0x9000 => {
                let skip_next_instruction = self.regs.V[usize::from(x)] != self.regs.V[usize::from(y)];

                if (skip_next_instruction) {
                    self.regs.PC += 2;
                }
            }
            // Annn - LD I, addr
            0xA000 => self.regs.I = nnn,
            // Bnnn - JP V0, addr
            0xB000 => self.regs.PC += nnn + (self.regs.V[0] as u16),
            // RND Vx, byte
            0xC000 => {
                let mut rand = rand::thread_rng();
                self.regs.V[usize::from(x)] = rand.gen::<u8>() & kk;
            }
            // DRW Vx, Vy, nibble
            0xD000 => {
                
            }
            0xE000 => match instruction & 0x00FF {
                // SKP Vx
                0x9E => panic!("Unexpected instructions {}", instruction),
                // SKNP Vx
                0xA1 => panic!("Unexpected instructions {}", instruction),
                _ => panic!("Unexpected instructions {}", instruction),
            }
            0xF000 => match instruction & 0x00FF {
                // LD Vx, DT
                0x07 => self.regs.V[usize::from(x)] = self.regs.DT,
                // LD Vx, K
                0x0A => {},
                // LD DT, Vx
                0x15 => self.regs.DT = self.regs.V[usize::from(x)],
                // LD ST, Vx
                0x18 => self.regs.ST = self.regs.V[usize::from(x)],
                // ADD I, Vx
                0x1E => self.regs.I += self.regs.V[usize::from(x)] as u16,
                // LD F, Vx
                0x29 => {
                    let size_of_sprite: u8 = 5;
                    let address: u16 = (self.regs.V[usize::from(x)] * size_of_sprite) as u16;

                    self.regs.I = address;
                },
                // LD B, Vx
                0x33 => {
                    let number = self.regs.V[usize::from(x)];
                    let h_digit = number / 100;
                    let t_digit = (number / 10) % 10;
                    let o_digit = number % 10;

                    let base_addr: usize = usize::from(self.regs.I);

                    self.ram[base_addr] = h_digit;
                    self.ram[base_addr + 1] = t_digit;
                    self.ram[base_addr + 2] = o_digit;
                },
                // LD [I], Vx
                0x55 => {
                    for offset in 0..16 {
                        self.ram[usize::from(self.regs.I + offset)] = self.regs.V[usize::from(offset)];
                    }
                }
                // LD Vx, [I]
                0x65 => {
                    for offset in 0..16 {
                        self.regs.V[usize::from(offset)] = self.ram[usize::from(self.regs.I + offset)];
                    }
                }
                _ => panic!("Unexceptected instructions {}", instruction)
            }
            _ => panic!("Unexceptected instructions {}", instruction)
        }

        self.regs.PC += 2
    }

    fn new_internal() -> Self {
        Chip8 {
            ram: [0; 4096],
            regs: Registers::default(),
            stack: Stack::default(),
            is_halt: false,
        }
    }

    fn load_sprites(ram: &mut [u8]) {
        for (i, byte) in SPRITES.iter().enumerate() {
            ram[i] = *byte
        }
    }
}

const SPRITES: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,
        0x20, 0x60, 0x20, 0x20, 0x70,
        0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0,
        0x90, 0x90, 0xF0, 0x10, 0x10,
        0x90, 0x90, 0xF0, 0x10, 0x10,
        0xF0, 0x80, 0xF0, 0x90, 0xF0,
        0xF0, 0x10, 0x20, 0x40, 0x40,
        0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0,
        0xF0, 0x90, 0xF0, 0x90, 0x90,
        0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0,
        0xE0, 0x90, 0x90, 0x90, 0xE0,
        0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
];
