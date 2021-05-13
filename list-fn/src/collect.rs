use super::*;

struct CollectState<I: ListFn> {
    end: Vec<I::Item>,
    input: I,
}

impl<I: ListFn> ListFn for CollectState<I> {
    type Item = ();
    type End = Vec<I::Item>;
    fn state(mut self) -> ListState<Self> {
        match self.input.state() {
            ListState::Some(first, next) => {
                self.end.push(first);
                ListState::Some(
                    (),
                    CollectState {
                        end: self.end,
                        input: next,
                    },
                )
            }
            ListState::End(..) => ListState::End(self.end),
        }
    }
}

pub trait Collect: ListFn {
    fn collect(self) -> Vec<Self::Item> {
        CollectState {
            end: Vec::new(),
            input: self,
        }
        .fold()
    }
}

impl<L: ListFn> Collect for L {}
