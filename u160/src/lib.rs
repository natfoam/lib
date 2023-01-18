pub struct U160(pub [u32; 5]);

impl U160 {
    pub fn set(&mut self, i: usize) {
        self.0[i / 8] |= 1 << (i % 8);
    }
}
