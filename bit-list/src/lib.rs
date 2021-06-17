use list_fn::{FlatMap, FlatMapFn, FlatMapList, ListFn, ListSome, ListState, ResultFn};
use std::marker::PhantomData;
use uints::UInt;

// LSB first bit vector.
pub struct BitVec<T: UInt> {
    array: T,
    size: u8,
}

impl<T: UInt> BitVec<T> {
    fn new(array: T) -> Self {
        BitVec {
            array,
            size: T::BITS,
        }
    }
}

impl<T: UInt> Default for BitVec<T> {
    fn default() -> Self {
        BitVec {
            array: T::ZERO,
            size: 0,
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
                next: BitVec {
                    array: self.array >> 1,
                    size: size - 1,
                },
            }),
        }
    }
}

pub struct Lsb0FlatMap<T: UInt>(PhantomData<T>);

impl<T: UInt> Default for Lsb0FlatMap<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: UInt> FlatMapFn for Lsb0FlatMap<T> {
    type Input = T;
    type OutputList = BitVec<T>;
    fn map(&self, item: T) -> BitVec<T> {
        BitVec::new(item)
    }
}

pub trait Lsb0
where
    Self: ListFn,
    Self::Item: UInt,
    Self::End: ResultFn<Result = ()>,
{
    fn lsb0(self) -> FlatMapList<Self, Lsb0FlatMap<Self::Item>> {
        self.flat_map(Lsb0FlatMap::default())
    }
}

impl<L> Lsb0 for L
where
    Self: ListFn,
    Self::Item: UInt,
    Self::End: ResultFn<Result = ()>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use list_fn::{Collect, IterWrap};
    #[test]
    fn it_works() {
        let a: &[u8] = &[1, 2];
        assert_eq!(
            a.iter().cloned().lsb0().collect().items,
            vec!(
                true, false, false, false, false, false, false, false, false, true, false, false,
                false, false, false, false
            )
        );
        let v: Vec<u8> = vec![3, 4];
        assert_eq!(
            v.into_iter().lsb0().collect().items,
            vec!(
                true, true, false, false, false, false, false, false, false, false, true, false,
                false, false, false, false
            )
        );
        {
            let mut r = Vec::new();
            let v: Vec<u8> = vec![3, 4];
            for elem in v.into_iter().lsb0().iter_wrap() {
                r.push(elem)
            }
            assert_eq!(
                r,
                vec!(
                    true, true, false, false, false, false, false, false, false, false, true,
                    false, false, false, false, false
                )
            );
        }
    }
}
