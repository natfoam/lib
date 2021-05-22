use crate::ListSome;

use super::*;

pub struct TakeList<L: ListFn> {
    next: L,
    count: usize,
}

impl<L: ListFn> ListFn for TakeList<L> {
    type Item = L::Item;
    type End = ();
    fn next(self) -> ListState<Self> {
        if self.count == 0 {
            ListState::End(())
        } else {
            match self.next.next() {
                ListState::Some(ListSome { first, next }) => ListState::Some(ListSome {
                    first,
                    next: TakeList {
                        next,
                        count: self.count - 1,
                    },
                }),
                ListState::End(_) => ListState::End(()),
            }
        }
    }
}

pub trait Take: ListFn {
    fn take(self, count: usize) -> TakeList<Self> {
        TakeList { next: self, count }
    }
}

impl<L: ListFn> Take for L {}
