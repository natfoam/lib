use super::*;

struct CollectState<I: ListFn> {
    result: Vec<I::Item>,
    input: I,
}

impl<I: ListFn> ListFn for CollectState<I> {
    type Item = ();
    type End = Vec<I::Item>;
    fn state(mut self) -> ListState<Self> {
        match self.input.state() {
            ListState::Some(first, next) => {
                self.result.push(first);
                ListState::Some(
                    (),
                    CollectState {
                        result: self.result,
                        input: next,
                    },
                )
            }
            ListState::End(..) => ListState::End(self.result),
        }
    }
}

pub trait Collect: ListFn {
    fn collect(self) -> Vec<Self::Item> {
        CollectState {
            result: Vec::new(),
            input: self,
        }
        .fold()
    }
}

impl<L: ListFn> Collect for L {}
