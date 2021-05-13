use list_fn::{Empty, FlatScan, FlatScanFn, FlatScanState, ListFn, ListState, ResultFn};
use std::marker::PhantomData;
use uints::UInt;

pub struct Lsb0List<T: UInt> {
    value: T,
    size: u8,
}

impl<T: UInt> Lsb0List<T> {
    fn new(value: T) -> Self {
        Lsb0List {
            value,
            size: T::BITS,
        }
    }
}

impl<T: UInt> ListFn for Lsb0List<T> {
    type Item = bool;
    type End = Lsb0FlatScan<T>;
    fn next(self) -> ListState<Self> {
        match self.size {
            0 => ListState::End(Default::default()),
            size => ListState::Some(
                self.value & T::ONE != T::ZERO,
                Lsb0List {
                    value: self.value >> 1,
                    size: size - 1,
                },
            ),
        }
    }
}

pub struct Lsb0FlatScan<T: UInt>(PhantomData<T>);

impl<T: UInt> Default for Lsb0FlatScan<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: UInt> FlatScanFn for Lsb0FlatScan<T> {
    type InputItem = T;
    type InputResult = ();
    type ItemList = Lsb0List<T>;
    type EndList = Empty<bool, ()>;
    fn item(self, item: T) -> Lsb0List<T> {
        Lsb0List::new(item)
    }
    fn end(self, _: ()) -> Empty<bool, ()> {
        Empty::new(())
    }
}

pub trait Lsb0
where
    Self: ListFn,
    Self::Item: UInt,
    Self::End: ResultFn<Result = ()>
{
    fn lsb0(self) -> FlatScanState<Self, Lsb0FlatScan<Self::Item>>
    {
        self.flat_scan(Lsb0FlatScan::default())
    }
}

impl<L> Lsb0 for L
where
    Self: ListFn,
    Self::Item: UInt,
    Self::End: ResultFn<Result = ()>
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
            a.iter().cloned().lsb0().collect(),
            vec!(
                true, false, false, false, false, false, false, false, false, true, false, false,
                false, false, false, false
            )
        );
        let v: Vec<u8> = vec![3, 4];
        assert_eq!(
            v.into_iter().lsb0().collect(),
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
