use list_fn::{Empty, FlatScanFn, Id, ListFn, ListSome, ListState};
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

pub const fn bit_vec16_new(array: u16, size: u8) -> BitVec16 {
    BitVec16 { array, size }
}

pub type BitVec64 = BitVec<u64>;

pub type BitVec32 = BitVec<u32>;

pub type BitVec16 = BitVec<u16>;

#[derive(Default)]
pub struct ByteList(BitVec32);

impl ListFn for ByteList {
    type Item = u8;
    type End = ByteList;
    fn next(self) -> ListState<Self> {
        if self.0.size < 8 {
            ListState::End(self)
        } else {
            ListState::Some(ListSome {
                first: self.0.array as u8,
                next: ByteList(BitVec::new(self.0.array >> 8, self.0.size - 8)),
            })
        }
    }
}

impl FlatScanFn for ByteList {
    type InputItem = BitVec16;
    type InputResult = ();
    type OutputList = Self;
    type EndList = Empty<u8, Id<BitVec32>>;
    fn map_item(self, item: BitVec16) -> Self::OutputList {
        ByteList(self.0.concat(BitVec32::new(item.array as u32, item.size)))
    }
    fn map_result(self, _: ()) -> Self::EndList {
        Empty::new(Id::new(self.0))
    }
}
