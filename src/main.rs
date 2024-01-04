use chip_8::Chip8;
use rom_reader::ROMReader;
use std::env;

mod chip_8;
mod rom_reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom_path = args.get(1).expect("ERROR: invalid ROM file path");

    let rom = ROMReader::read(rom_path);

    let mut chip_8 = Chip8::new(&rom);

    loop {
        chip_8.emulate_cycle();
        if chip_8.is_halt {
            break;
        }
    }
}
