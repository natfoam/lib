use lim_bit_vec::BitVec;
use list_fn::{FlatMap, FlatMapFn, FlatMapList, ListFn, ResultFn};
use std::marker::PhantomData;
use uints::UInt;

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
        BitVec::new_full(item)
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
            a.iter().cloned().lsb0().collect(Vec::default()).collection,
            vec!(
                true, false, false, false, false, false, false, false, false, true, false, false,
                false, false, false, false
            )
        );
        let v: Vec<u8> = vec![3, 4];
        assert_eq!(
            v.into_iter().lsb0().collect(Vec::default()).collection,
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
