use registers::Registers;
use stack::Stack;

mod registers;
mod stack;

pub struct Chip8 {
    ram: [u8; 4096],
    regs: Registers,
    stack: Stack,
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

        this
    }

    fn new_internal() -> Self {
        Chip8 {
            ram: [0; 4096],
            regs: Registers::default(),
            stack: Stack::default(),
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
