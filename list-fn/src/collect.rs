use std::marker::PhantomData;

use super::*;

pub struct CollectState<T, E>(Vec<T>, PhantomData<E>);

impl<T, E> ScanFn for CollectState<T, E> {
    type InputItem = T;
    type InputResult = E;
    type OutputItem = ();
    type OutputResult = Vec<T>;

    fn map_input(mut self, input: Self::InputItem) -> ScanState<Self> {
        self.0.push(input);
        ScanState {
            first: (),
            next: self,
        }
    }

    fn map_result(self, _: Self::InputResult) -> Self::OutputResult {
        self.0
    }
}

pub trait Collect
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn collect(self) -> Vec<Self::Item> {
        self.scan(CollectState(Vec::new(), PhantomData::default()))
            .fold()
    }
}

impl<L> Collect for L
where
    Self: ListFn,
    Self::End: ResultFn,
{
}
