use super::*;

pub trait Fold: ListFn {
    fn fold(mut self) -> Self::End {
        loop {
            match self.next() {
                ListState::Some(_, next) => self = next,
                ListState::End(end) => return end,
            }
        }
    }
}

impl<S: ListFn> Fold for S {}
