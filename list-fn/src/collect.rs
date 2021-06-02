use std::marker::PhantomData;

use super::*;

struct CollectState<T, E>(Vec<T>, PhantomData<E>);

pub struct CollectResult<I, R> {
    pub items: Vec<I>,
    pub result: R,
}

impl<T, E> ScanFn for CollectState<T, E> {
    type InputItem = T;
    type InputResult = E;
    type OutputItem = ();
    type OutputResult = CollectResult<T, Self::InputResult>;

    fn map_input(mut self, input: Self::InputItem) -> ScanState<Self> {
        self.0.push(input);
        ScanState {
            first: (),
            next: self,
        }
    }

    fn map_result(self, result: Self::InputResult) -> Self::OutputResult {
        CollectResult { items: self.0, result }
    }
}

pub trait Collect
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn collect(self) -> CollectResult<Self::Item, <Self::End as ResultFn>::Result> {
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
