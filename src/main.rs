use std::env;
use rom_reader::ROMReader;
use chip_8::Chip8;

mod rom_reader;
mod chip_8;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom_path = args.get(1)
        .expect("ERROR: invalid ROM file path");

    let rom = ROMReader::read(rom_path);

    let chip_8 = Chip8::new(&rom);
}
