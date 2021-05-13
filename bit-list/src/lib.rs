use list_fn::{Empty, FlatScan, FlatScanEx, FlatScanListFn, List, ListFn, ResultFn};
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
    type End = Lsb0<T>;
    fn list(self) -> List<Self> {
        match self.size {
            0 => List::End(Default::default()),
            size => List::Some(
                self.value & T::ONE != T::ZERO,
                Lsb0List {
                    value: self.value >> 1,
                    size: size - 1,
                },
            ),
        }
    }
}

pub struct Lsb0<T: UInt>(PhantomData<T>);

impl<T: UInt> Default for Lsb0<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: UInt> FlatScan for Lsb0<T> {
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

pub trait BitsEx: ListFn
where
    Self::Item: UInt,
{
    fn lsb0(self) -> FlatScanListFn<Self, Lsb0<Self::Item>>
    where
        Self::End: ResultFn<Result = ()>,
    {
        self.flat_scan(Default::default())
    }
}

impl<L: ListFn> BitsEx for L where Self::Item: UInt {}

#[cfg(test)]
mod tests {
    use super::*;
    use list_fn::{Collect, IntoIterWrap};
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
