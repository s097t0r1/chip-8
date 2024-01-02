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

    fn new_internal() -> Self {
        Chip8 {
            ram: [0; 4096],
            regs: Registers::default(),
            stack: Stack::default(),
        }
    }

    pub fn new(rom: &Vec<u8>) -> Self {
        let offset = 0x200;

        let mut this = Chip8::new_internal();

        // Initialize RAM with ROM 
        for (i, value) in rom.iter().enumerate() {
            this.ram[i + offset] = dbg!(*value);
        }

        this
    }
}
