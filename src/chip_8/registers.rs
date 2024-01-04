#[derive(Default)]
pub struct Registers {
    pub V: [u8; 16],
    pub DT: u8,
    pub ST: u8,
    pub I: u16,
    pub PC: u16,
}
