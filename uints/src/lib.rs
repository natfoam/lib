#![no_std]

use core::fmt::Debug;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, Shl, ShlAssign, Shr, ShrAssign, Sub,
};
use fixed_array::Array;

pub trait Common: Default + PartialOrd + Debug {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    type Array: Array<Output = u8>;
    const BYTES: u8 = Self::Array::SIZE as u8;
    const BITS: u8 = Self::BYTES * 8;
    fn leading_zeros(&self) -> u8;
    fn trailing_zeros(&self) -> u8;
    fn count_ones(&self) -> u8;
    fn log2(&self) -> u8 {
        Self::BITS - 1 - self.leading_zeros()
    }
}

pub trait Lsb0Array: Common {
    fn lsb0_array(&self) -> Self::Array;
}

pub trait UInt:
    Copy
    + Ord
    + Eq
    + Common
    + Lsb0Array
    + Sub<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + Shl<u8, Output = Self>
    + ShlAssign<u8>
    + Shr<u8, Output = Self>
    + ShrAssign<u8>
    + BitOr<Output = Self>
    + BitOrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
{
    fn remove(self, i: u8) -> Self {
        if i < Self::BITS {
            self >> i
        } else {
            Self::ZERO
        }
    }
    fn take(self, i: u8) -> Self {
        if i == 0 {
            self
        } else {
            let d = Self::BITS - i;
            self << d >> d
        }
    }
    fn ror(self, i: u32) -> Self;
    fn overflow_add(self, v: Self) -> Self;
}

pub trait Number: Common {
    fn set(&mut self, i: u8);
    fn unset(&mut self, i: u8);
    fn is_set(&self, i: u8) -> bool;
    fn add(&mut self, v: u32);
    fn subtract(&mut self, v: u8);
    fn mask(&mut self, m: u8) -> u8;
}

impl Common for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = Self::MAX;
    type Array = [u8; 1];
    fn leading_zeros(&self) -> u8 {
        Self::leading_zeros(*self) as u8
    }
    fn trailing_zeros(&self) -> u8 {
        Self::trailing_zeros(*self) as u8
    }
    fn count_ones(&self) -> u8 {
        Self::count_ones(*self) as u8
    }
}

impl Lsb0Array for u8 {
    fn lsb0_array(&self) -> [u8; 1] {
        [*self]
    }
}

impl UInt for u8 {
    fn ror(self, i: u32) -> Self {
        self.rotate_right(i)
    }
    fn overflow_add(self, v: Self) -> Self {
        self.overflowing_add(v).0
    }
}

pub type U16 = [u8; 2];

pub const fn u16_new(v: u16) -> U16 {
    [v as u8, (v >> 8) as u8]
}

impl Common for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = Self::MAX;
    type Array = [u8; 2];
    fn leading_zeros(&self) -> u8 {
        Self::leading_zeros(*self) as u8
    }
    fn trailing_zeros(&self) -> u8 {
        Self::trailing_zeros(*self) as u8
    }
    fn count_ones(&self) -> u8 {
        Self::count_ones(*self) as u8
    }
}

impl Lsb0Array for u16 {
    fn lsb0_array(&self) -> [u8; 2] {
        let x = u16_new(*self);
        [x[0], x[1]]
    }
}

impl UInt for u16 {
    fn ror(self, i: u32) -> Self {
        self.rotate_right(i)
    }
    fn overflow_add(self, v: Self) -> Self {
        self.overflowing_add(v).0
    }
}

pub type U32 = [U16; 2];

pub const fn u32_new(v: u32) -> U32 {
    [u16_new(v as u16), u16_new((v >> 16) as u16)]
}

impl Common for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = Self::MAX;
    type Array = [u8; 4];
    fn leading_zeros(&self) -> u8 {
        Self::leading_zeros(*self) as u8
    }
    fn trailing_zeros(&self) -> u8 {
        Self::trailing_zeros(*self) as u8
    }
    fn count_ones(&self) -> u8 {
        Self::count_ones(*self) as u8
    }
}

impl Lsb0Array for u32 {
    fn lsb0_array(&self) -> [u8; 4] {
        let x = u32_new(*self);
        [x[0][0], x[0][1], x[1][0], x[1][1]]
    }
}

impl UInt for u32 {
    fn ror(self, i: u32) -> Self {
        self.rotate_right(i)
    }
    fn overflow_add(self, v: Self) -> Self {
        self.overflowing_add(v).0
    }
}

pub type U64 = [U32; 2];

pub const fn u64_new(v: u64) -> U64 {
    [u32_new(v as u32), u32_new((v >> 32) as u32)]
}

impl Common for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = Self::MAX;
    type Array = [u8; 8];
    fn leading_zeros(&self) -> u8 {
        Self::leading_zeros(*self) as u8
    }
    fn trailing_zeros(&self) -> u8 {
        Self::trailing_zeros(*self) as u8
    }
    fn count_ones(&self) -> u8 {
        Self::count_ones(*self) as u8
    }
}

impl Lsb0Array for u64 {
    fn lsb0_array(&self) -> [u8; 8] {
        let x = u64_new(*self);
        [
            x[0][0][0], //
            x[0][0][1], //
            x[0][1][0], //
            x[0][1][1], //
            x[1][0][0], //
            x[1][0][1], //
            x[1][1][0], //
            x[1][1][1], //
        ]
    }
}

impl UInt for u64 {
    fn ror(self, i: u32) -> Self {
        self.rotate_right(i)
    }
    fn overflow_add(self, v: Self) -> Self {
        self.overflowing_add(v).0
    }
}

pub type U128 = [U64; 2];

pub const fn u128_new(v: u128) -> U128 {
    [u64_new(v as u64), u64_new((v >> 64) as u64)]
}

impl Common for u128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = Self::MAX;
    type Array = [u8; 16];
    fn leading_zeros(&self) -> u8 {
        Self::leading_zeros(*self) as u8
    }
    fn trailing_zeros(&self) -> u8 {
        Self::trailing_zeros(*self) as u8
    }
    fn count_ones(&self) -> u8 {
        Self::count_ones(*self) as u8
    }
}

impl Lsb0Array for u128 {
    fn lsb0_array(&self) -> [u8; 16] {
        let x = u128_new(*self);
        [
            x[0][0][0][0],
            x[0][0][0][1],
            x[0][0][1][0],
            x[0][0][1][1],
            x[0][1][0][0],
            x[0][1][0][1],
            x[0][1][1][0],
            x[0][1][1][1],
            x[1][0][0][0],
            x[1][0][0][1],
            x[1][0][1][0],
            x[1][0][1][1],
            x[1][1][0][0],
            x[1][1][0][1],
            x[1][1][1][0],
            x[1][1][1][1],
        ]
    }
}

impl UInt for u128 {
    fn ror(self, i: u32) -> Self {
        self.rotate_right(i)
    }
    fn overflow_add(self, v: Self) -> Self {
        self.overflowing_add(v).0
    }
}

impl Number for u8 {
    fn unset(&mut self, i: u8) {
        *self &= !(1 << i)
    }
    fn set(&mut self, i: u8) {
        *self |= 1 << i;
    }
    fn is_set(&self, i: u8) -> bool {
        (*self >> i) & 1 != 0
    }
    fn add(&mut self, v: u32) {
        *self += v as u8;
    }
    fn subtract(&mut self, v: u8) {
        *self -= v;
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = *self & m;
        *self &= u8::MAX - m;
        result
    }
}

impl Number for u16 {
    fn unset(&mut self, i: u8) {
        *self &= !(1 << i)
    }
    fn set(&mut self, i: u8) {
        *self |= 1 << i;
    }
    fn is_set(&self, i: u8) -> bool {
        (self >> i) & 1 != 0
    }
    fn add(&mut self, v: u32) {
        *self += v as Self;
    }
    fn subtract(&mut self, v: u8) {
        *self -= v as Self;
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = *self & m as Self;
        *self &= Self::MAX - m as Self;
        result as u8
    }
}

impl Number for u32 {
    fn unset(&mut self, i: u8) {
        *self &= !(1 << i)
    }
    fn set(&mut self, i: u8) {
        *self |= 1 << i;
    }
    fn is_set(&self, i: u8) -> bool {
        (self >> i) & 1 != 0
    }
    fn add(&mut self, v: u32) {
        *self += v as Self;
    }
    fn subtract(&mut self, v: u8) {
        *self -= v as Self;
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = *self & m as Self;
        *self &= Self::MAX - m as Self;
        result as u8
    }
}

impl Number for u64 {
    fn unset(&mut self, i: u8) {
        *self &= !(1 << i)
    }
    fn set(&mut self, i: u8) {
        *self |= 1 << i;
    }
    fn is_set(&self, i: u8) -> bool {
        (self >> i) & 1 != 0
    }
    fn add(&mut self, v: u32) {
        *self += v as Self;
    }
    fn subtract(&mut self, v: u8) {
        *self -= v as Self;
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = *self & m as Self;
        *self &= Self::MAX - m as Self;
        result as u8
    }
}

impl Number for u128 {
    fn unset(&mut self, i: u8) {
        *self &= !(1 << i)
    }
    fn set(&mut self, i: u8) {
        *self |= 1 << i;
    }
    fn is_set(&self, i: u8) -> bool {
        (self >> i) & 1 != 0
    }
    fn add(&mut self, v: u32) {
        *self += v as Self;
    }
    fn subtract(&mut self, v: u8) {
        *self -= v as Self;
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = *self & m as Self;
        *self &= Self::MAX - m as Self;
        result as u8
    }
}

impl Number for usize {
    fn unset(&mut self, i: u8) {
        *self &= !(1 << i)
    }
    fn set(&mut self, i: u8) {
        *self |= 1 << i;
    }
    fn is_set(&self, i: u8) -> bool {
        (self >> i) & 1 != 0
    }
    fn add(&mut self, v: u32) {
        *self += v as Self;
    }
    fn subtract(&mut self, v: u8) {
        *self -= v as Self;
    }
    fn mask(&mut self, m: u8) -> u8 {
        let result = *self & m as Self;
        *self &= Self::MAX - m as Self;
        result as u8
    }
}

impl Common for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = Self::MAX;
    type Array = [u8; (Self::BITS / 8) as usize];
    fn leading_zeros(&self) -> u8 {
        Self::leading_zeros(*self) as u8
    }
    fn trailing_zeros(&self) -> u8 {
        Self::trailing_zeros(*self) as u8
    }
    fn count_ones(&self) -> u8 {
        Self::count_ones(*self) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn log2() {
        assert_eq!(1u32.log2(), 0);
        assert_eq!(2u32.log2(), 1);
        assert_eq!(3u32.log2(), 1);
        assert_eq!(4u32.log2(), 2);
    }
}
