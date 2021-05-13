use super::*;

pub struct Take<L: ListFn> {
    input: L,
    count: usize,
}

impl<L: ListFn> ListFn for Take<L> {
    type Item = L::Item;
    type End = ();
    fn state(self) -> ListState<Self> {
        if self.count == 0 {
            ListState::End(())
        } else {
            match self.input.state() {
                ListState::Some(first, input) => ListState::Some(
                    first,
                    Take {
                        input,
                        count: self.count - 1,
                    },
                ),
                ListState::End(_) => ListState::End(()),
            }
        }
    }
}

pub trait TakeEx: ListFn {
    fn take(self, count: usize) -> Take<Self> {
        Take { input: self, count }
    }
}

impl<L: ListFn> TakeEx for L {}
