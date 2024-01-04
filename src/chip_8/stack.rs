#[derive(Default)]
pub struct Stack {
    S: [u16; 32],
    SP: u16,
}

impl Stack {
    
    pub fn push(&mut self, address: u16) {
        self.S[usize::from(self.SP)] = address;
        self.SP += 1;
    }

    pub fn pop(&mut self) -> u16 {
        let address = self.S[usize::from(self.SP)];
        self.SP -= 1;

        address
    }
}
