use super::*;
use std::marker::PhantomData;

pub struct Empty<T, E>(E, PhantomData<T>);

impl<T, E> ListFn for Empty<T, E> {
    type Item = T;
    type End = Self;
    fn list(self) -> List<Self> {
        List::End(self)
    }
}

impl<T, E> Empty<T, E> {
    pub fn new(e: E) -> Self {
        Empty(e, PhantomData::default())
    }
}

impl<T, E> ResultFn for Empty<T, E> {
    type Result = E;
    fn result(self) -> E {
        self.0
    }
}
