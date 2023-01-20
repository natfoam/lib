use core::marker::PhantomData;

use super::*;

struct CollectState<C, E>(C, PhantomData<E>);

impl<C, E> CollectState<C, E> {
    const fn new(c: C) -> Self {
        CollectState(c, PhantomData {})
    }
}

pub struct CollectResult<C, R> {
    pub collection: C,
    pub result: R,
}

impl<C: Collection, E> ScanFn for CollectState<C, E> {
    type InputItem = C::Item;
    type InputResult = E;
    type OutputItem = ();
    type OutputResult = CollectResult<C, Self::InputResult>;

    fn map_input(self, input: Self::InputItem) -> ScanState<Self> {
        ScanState {
            first: (),
            next: CollectState::new(self.0.add(input)),
        }
    }

    fn map_result(self, result: Self::InputResult) -> Self::OutputResult {
        CollectResult {
            collection: self.0,
            result,
        }
    }
}

pub trait Collect
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn collect<C: Collection<Item = Self::Item>>(
        self,
        c: C,
    ) -> CollectResult<C, <Self::End as ResultFn>::Result> {
        self.scan(CollectState::new(c)).fold().result()
    }
}

impl<L> Collect for L
where
    Self: ListFn,
    Self::End: ResultFn,
{
}
