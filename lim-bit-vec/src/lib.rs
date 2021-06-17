use list_fn::{ListFn, ListSome, ListState};
use uints::UInt;

// LSB first bit vector.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BitVec<T: UInt> {
    pub array: T,
    pub size: u8,
}

impl<T: UInt> BitVec<T> {
    pub fn new_full(array: T) -> Self {
        Self::new(array, T::BITS)
    }
    pub fn new(array: T, size: u8) -> Self {
        BitVec { array, size }
    }
    pub fn concat(self, v: Self) -> Self {
        BitVec {
            array: self.array | (v.array << self.size),
            size: self.size + v.size,
        }
    }
}

impl<T: UInt> ListFn for BitVec<T> {
    type Item = bool;
    type End = ();
    fn next(self) -> ListState<Self> {
        match self.size {
            0 => ListState::End(()),
            size => ListState::Some(ListSome {
                first: self.array & T::ONE != T::ZERO,
                next: BitVec::new(self.array >> 1, size - 1),
            }),
        }
    }
}