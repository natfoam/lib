use super::*;

pub struct TakeList<L: ListFn> {
    input: L,
    count: usize,
}

impl<L: ListFn> ListFn for TakeList<L> {
    type Item = L::Item;
    type End = ();
    fn state(self) -> ListState<Self> {
        if self.count == 0 {
            ListState::End(())
        } else {
            match self.input.state() {
                ListState::Some(first, input) => ListState::Some(
                    first,
                    TakeList {
                        input,
                        count: self.count - 1,
                    },
                ),
                ListState::End(_) => ListState::End(()),
            }
        }
    }
}

pub trait Take: ListFn {
    fn take(self, count: usize) -> TakeList<Self> {
        TakeList { input: self, count }
    }
}

impl<L: ListFn> Take for L {}
