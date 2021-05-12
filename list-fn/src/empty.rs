use super::*;
use std::marker::PhantomData;

pub struct Empty<T, E>(E, PhantomData<T>);

impl<T, E> ListFn for Empty<T, E> {
    type Item = T;
    type End = E;
    fn list(self) -> List<Self> {
        List::End(self.0)
    }
}

impl<T, E> Empty<T, E> {
    pub fn new(e: E) -> Self {
        Empty(e, PhantomData::default())
    }
}
