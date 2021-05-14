use super::*;
use std::marker::PhantomData;

pub struct FlattenList<I>(PhantomData<I>);

impl<I> Default for FlattenList<I> {
    fn default() -> Self {
        FlattenList(PhantomData::default())
    }
}

impl<I> FlatMapFn for FlattenList<I>
where
    I: ListFn,
    I::Item: ListFn,
{
    type Input = I::Item;
    type OutputList = I::Item;
    fn map(&self, input: Self::Input) -> Self::OutputList {
        input
    }
}

pub trait Flatten
where
    Self: ListFn,
    Self::Item: ListFn,
{
    fn flatten(self) -> FlatMapList<Self, FlattenList<Self>> {
        self.flat_map(FlattenList::default())
    }
}

impl<S> Flatten for S
where
    Self: ListFn,
    Self::Item: ListFn,
{
}
