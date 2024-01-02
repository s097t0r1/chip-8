#[derive(Default)]
pub struct Registers {
    V: [u8; 16],
    DT: u8,
    ST: u8,
    I: u16,
    PC: u16,
}
