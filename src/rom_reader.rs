use std::{fs::File, io::Read};

pub struct ROMReader;

impl ROMReader {

    pub fn read(rom_path: &String) -> Vec<u8> {

        let mut rom_file = File::open(rom_path)
            .expect("ERROR: cannot open ROM file");

        let mut buffer = Vec::new();
        rom_file.read_to_end(&mut buffer)
            .expect("ERROR: cannot read ROM file");

        buffer
    }
}