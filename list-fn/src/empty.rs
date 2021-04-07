use super::*;
use std::marker::PhantomData;

pub struct Empty<T>(PhantomData<T>);

impl<T> ListFn for Empty<T> {
    type Item = T;
    type End = Self;
    fn list(self) -> List<Self> {
        List::End(self)
    }
}

impl<T> Default for Empty<T> {
    fn default() -> Self {
        Empty(PhantomData::default())
    }
}
