use super::*;

pub trait Fold: ListFn {
    fn fold(mut self) -> Self::End {
        loop {
            match self.list() {
                List::Some(_, next) => self = next,
                List::End(end) => return end,
            }
        }
    }
}

impl<S: ListFn> Fold for S {}
