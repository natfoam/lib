use super::*;

struct CollectVec<I: ListFn> {
    end: Vec<I::Item>,
    input: I,
}

impl<I: ListFn> ListFn for CollectVec<I> {
    type Item = ();
    type End = Vec<I::Item>;
    fn list(mut self) -> List<Self> {
        match self.input.list() {
            List::Some(first, next) => {
                self.end.push(first);
                List::Some(
                    (),
                    CollectVec {
                        end: self.end,
                        input: next,
                    },
                )
            }
            List::End(..) => List::End(self.end),
        }
    }
}

pub trait Collect: ListFn {
    fn collect(self) -> Vec<Self::Item> {
        CollectVec {
            end: Vec::new(),
            input: self,
        }
        .fold()
    }
}

impl<L: ListFn> Collect for L {}
