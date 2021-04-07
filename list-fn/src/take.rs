use super::*;

pub struct Take<L: ListFn> {
    input: L,
    count: usize,
}

impl<L: ListFn> ListFn for Take<L> {
    type Item = L::Item;
    type End = ();
    fn list(self) -> List<Self> {
        if self.count == 0 {
            List::End(())
        } else {
            match self.input.list() {
                List::Some(first, input) => List::Some(first, Take { input, count: self.count - 1 }),
                List::End(_) => List::End(()),
            }
        }
    }
}

pub trait TakeEx: ListFn {
    fn take(self, count: usize) -> Take<Self> { Take { input: self, count }}
}

impl<L: ListFn> TakeEx for L {}