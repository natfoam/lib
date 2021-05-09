use uints::{Common, Number, u128_new, u16_new};

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct U144 {
    hi: u128,
    lo: u16,
}

impl U144 {
    pub const fn new(hi: u128, lo: u16) -> Self {
        Self { hi, lo }
    }
    pub const fn hi(&self) -> u128 {
        self.hi
    }
    pub const fn lo(&self) -> u16 {
        self.lo
    }
}

impl Common for U144 {
    const ZERO: U144 = U144 { hi: 0, lo: 0 };
    const ONE: U144 = U144 { hi: 0, lo: 1 };
    const MAX: U144 = U144 {
        hi: u128::MAX,
        lo: u16::MAX,
    };
    const BITS: u8 = 144;
    type Array = [u8; 18];
    fn leading_zeros(&self) -> u8 {
        match self.hi.leading_zeros() {
            128 => self.lo.leading_zeros() as u8 + 128,
            value => value as u8,
        }
    }
    fn trailing_zeros(&self) -> u8 {
        match self.lo.trailing_zeros() {
            16 => self.hi.trailing_zeros() as u8 + 16,
            value => value as u8,
        }
    }
    fn count_ones(&self) -> u8 {
        (self.hi.count_ones() + self.lo.count_ones()) as u8
    }
    fn lsb0_array(&self) -> [u8; 18] {
        let lo = u16_new(self.lo);
        let hi = u128_new(self.hi);
        [
            lo[0],
            lo[1],
            hi[0][0][0][0],
            hi[0][0][0][1],
            hi[0][0][1][0],
            hi[0][0][1][1],
            hi[0][1][0][0],
            hi[0][1][0][1],
            hi[0][1][1][0],
            hi[0][1][1][1],
            hi[1][0][0][0],
            hi[1][0][0][1],
            hi[1][0][1][0],
            hi[1][0][1][1],
            hi[1][1][0][0],
            hi[1][1][0][1],
            hi[1][1][1][0],
            hi[1][1][1][1],
        ]
    }
}

impl Number for U144 {
    fn add(&mut self, value: u32) {
        let lo = self.lo as u32 + value;
        self.hi += (lo >> 16) as u128;
        self.lo = lo as u16;
    }
    fn subtract(&mut self, value: u8) {
        self.add(0x10000 - value as u32);
        self.hi -= 1;
    }
    fn unset(&mut self, i: u8) {
        if i < 16 {
            self.lo &= !(1 << i);
        } else {
            self.hi &= !(1 << (i - 16));
        }
    }
    fn set(&mut self, i: u8) {
        if i < 16 {
            self.lo |= 1 << i;
        } else {
            self.hi |= 1 << (i - 16);
        }
    }
    fn is_set(&mut self, i: u8) -> bool {
        if i < 16 {
            (self.lo >> i) & 1 != 0
        } else {
            (self.hi >> (i - 16)) & 1 != 0
        }
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = self.lo & m as u16;
        self.lo &= u16::MAX - m as u16;
        result as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leading_zeros() {
        assert_eq!(U144::default().leading_zeros(), 144);
        assert_eq!(U144::new(u128::MAX, 0).leading_zeros(), 0);
        assert_eq!(U144::new(1, 0).leading_zeros(), 127);
        assert_eq!(U144::new(0, 1).leading_zeros(), 143);
        assert_eq!(U144::new(0, 0x8000).leading_zeros(), 128);
    }
    #[test]
    fn ord() {
        assert_eq!(U144::new(7, 0) < U144::new(0, 7), false);
        assert_eq!(U144::new(0, 0) < U144::new(0, 0), false);
        assert_eq!(U144::new(3, 7) < U144::new(3, 7), false);
        assert_eq!(U144::new(0, 0) < U144::new(0, 1), true);
        assert_eq!(U144::new(3, 7) < U144::new(3, 8), true);
        assert_eq!(U144::new(3, 7) < U144::new(4, 0), true);
    }
    #[test]
    fn add() {
        let mut v = U144::default();
        v.add(0xFFFF);
        v.add(0xFFFF);
        assert_eq!(v.hi(), 1);
        assert_eq!(v.lo(), 0xFFFE);
        v.add(1);
        assert_eq!(v.hi(), 1);
        assert_eq!(v.lo(), 0xFFFF);
        v.add(1);
        assert_eq!(v.hi(), 2);
        assert_eq!(v.lo(), 0);
    }
    #[test]
    fn subtract() {
        let mut v = U144::new(0x10, 0x0);
        v.subtract(0xFF);
        assert_eq!(v.hi(), 0xF);
        assert_eq!(v.lo(), 0xFF01);
        v.subtract(1);
        assert_eq!(v.hi(), 0xF);
        assert_eq!(v.lo(), 0xFF00);
    }
    #[test]
    fn set() {
        let mut v = U144::default();
        v.set(0);
        assert_eq!(v.hi, 0);
        assert_eq!(v.lo, 1);
        v.set(15);
        assert_eq!(v.hi, 0);
        assert_eq!(v.lo, 0x8001);
        v.set(16);
        assert_eq!(v.hi, 1);
        assert_eq!(v.lo, 0x8001);
        v.set(32);
        assert_eq!(v.hi, 0x10001);
        assert_eq!(v.lo, 0x8001);
    }
    #[test]
    fn trailing_zeros() {
        assert_eq!(U144::new(0x56, 0x100).trailing_zeros(), 8);
        assert_eq!(U144::new(0x56, 0).trailing_zeros(), 17);
        assert_eq!(U144::default().trailing_zeros(), 144);
    }
    #[test]
    fn count_ones() {
        assert_eq!(U144::new(0x56, 0x100).count_ones(), 5);
        assert_eq!(U144::new(0x56, 0).count_ones(), 4);
        assert_eq!(U144::default().count_ones(), 0);
    }
}
