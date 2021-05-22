use crate::ListSome;

use super::*;

struct CollectState<I: ListFn> {
    result: Vec<I::Item>,
    next: I,
}

impl<I: ListFn> ListFn for CollectState<I> {
    type Item = ();
    type End = Vec<I::Item>;
    fn next(mut self) -> ListState<Self> {
        match self.next.next() {
            ListState::Some(ListSome { first, next }) => {
                self.result.push(first);
                ListState::Some(ListSome {
                    first: (),
                    next: CollectState {
                        result: self.result,
                        next,
                    },
                })
            }
            ListState::End(..) => ListState::End(self.result),
        }
    }
}

pub trait Collect: ListFn {
    fn collect(self) -> Vec<Self::Item> {
        CollectState {
            result: Vec::new(),
            next: self,
        }
        .fold()
    }
}

impl<L: ListFn> Collect for L {}
