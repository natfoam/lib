use super::*;

pub trait Fold: ListFn {
    fn fold(mut self) -> Self::End {
        loop {
            match self.next() {
                ListState::Some(some) => self = some.next,
                ListState::End(end) => return end,
            }
        }
    }
}

impl<S: ListFn> Fold for S {}
