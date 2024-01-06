use rand::Rng;
use registers::Registers;
use stack::Stack;

mod registers;
mod stack;

pub struct Chip8 {
    ram: [u8; 4096],
    display: [[bool; 64]; 32],
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

        this.regs.PC = 0x200;

        this
    }

    pub fn tick(&mut self) -> Command {
        let instruction: u16 = ((self.ram[usize::from(self.regs.PC)] as u16) << 8) | self.ram[usize::from(self.regs.PC + 1)] as u16;
        let nnn: u16 = instruction & 0x0FFF;
        let n: u8 = instruction as u8 & 0x0F;
        let x: u8 = (instruction >> 8) as u8 & 0x0F;
        let y: u8 = (instruction as u8 >> 4) & 0x0F;
        let kk: u8 = instruction as u8;

        match instruction & 0xF000 {
            0x0000 => {
                match instruction & 0x00FF {
                    // CLS
                    0x00E0 => {
                        for i in 0..self.display.len() - 1 {
                            for j in 0..self.display[i].len() - 1 {
                                self.display[i][j] = false;
                            }
                        }
                    }
                    // RET
                    0x00EE => {
                        let address: u16 = self.stack.pop();
                        self.regs.PC = address;
                    }
                    _ =>  {
                        self.is_halt = true;
                    }
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
                self.regs.V[usize::from(x)] = self.regs.V[usize::from(x)].wrapping_add(kk); 
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
                    let x_value: u8 = self.regs.V[usize::from(x)];

                    self.regs.V[usize::from(x)] = x_value.wrapping_add(self.regs.V[usize::from(y)]);
                    self.regs.V[0xF] = if x < self.regs.V[usize::from(x)] { 1 } else { 0 };
                }
                // SUB Vx, Vy
                0x0005 => {
                    let x_value: u8 = self.regs.V[usize::from(x)];

                    self.regs.V[usize::from(x)] = x.wrapping_sub(self.regs.V[usize::from(y)]);
                    self.regs.V[0xF] = if x_value > self.regs.V[usize::from(x)] { 1 } else { 0 };
                }
                // SHR Vx {, Vy}
                0x0006 => {
                    let x_value: u8 = self.regs.V[usize::from(x)].wrapping_shr(1);
                    let overflow_flag: u8 = self.regs.V[usize::from(x)] & 0x1;

                    self.regs.V[0xF] = overflow_flag;
                    self.regs.V[usize::from(x)] = x_value;
                }
                // SUBN Vx, Vy
                0x0007 => {
                    let not_borrow: u8 = (self.regs.V[usize::from(y)] > self.regs.V[usize::from(x)]) as u8;

                    self.regs.V[usize::from(x)] = self.regs.V[usize::from(y)] - self.regs.V[usize::from(x)];
                    self.regs.V[0xF] = not_borrow;
                }
                // SHL Vx {, Vy}
                0x000E => {
                    let x_value: u8 = self.regs.V[usize::from(x)].wrapping_shl(1);
                    let overflow_flag: u8 = self.regs.V[usize::from(x)] & 0x80;

                    self.regs.V[0xF] = overflow_flag;
                    self.regs.V[usize::from(x)] = x_value;
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
                let start_x = self.regs.V[usize::from(x)];
                let start_y = self.regs.V[usize::from(y)];
                let mut is_collision_detected = false;

                for y_offset in 0..n {
                    let y = if start_y + y_offset >= DISPLAY_HEIGHT { start_y + y_offset - DISPLAY_HEIGHT } else { start_y + y_offset };
                    let sprite_row = self.ram[usize::from(self.regs.I + (u16::from(y_offset)))];

                    for x_offset in 0..SPRITE_WIDTH {
                        let x = if start_x + x_offset >= DISPLAY_WIDTH { start_x + x_offset - DISPLAY_WIDTH } else { start_x + x_offset };

                        let is_sprite_pixel_colored = ((sprite_row).wrapping_shr((8 - x_offset - 1) as u32) & 1) > 0;
                        let is_display_pixel_colored = self.display[usize::from(y)][usize::from(x)];

                        let is_draw_pixel = is_display_pixel_colored ^ is_sprite_pixel_colored;
                        is_collision_detected = is_display_pixel_colored & is_sprite_pixel_colored;

                        self.display[usize::from(y)][usize::from(x)] = is_draw_pixel;
                    }
                }

                Chip8::debug_drawing_screen(&self.display);

                self.regs.V[0xF] = is_collision_detected.into();
                self.regs.PC += 2;

                return Command::Draw(self.display);
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

        self.regs.PC += 2;

        Command::Nothing
    }

    fn debug_drawing_screen(screen: &[[bool; 64]; 32]) {
        for row in screen.iter() {
            for pixel in row.iter() {
                if *pixel { print!("1") } else { print!("0") }
            }
            println!();
        }

        println!("\n");
    }

    fn new_internal() -> Self {
        Chip8 {
            ram: [0; 4096],
            display: [[false; 64]; 32],
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

pub enum Command {
    Nothing,
    Draw([[bool; 64]; 32]),
    Beep,
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

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

const SPRITE_WIDTH: u8 = 8;