use super::*;

pub trait FlatMapFn {
    type Input;
    type OutputList: ListFn;
    /// Map the given `input` item into a list.
    fn map(&self, input: Self::Input) -> Self::OutputList;
}

pub struct FlatMapList<I: ListFn, F: FlatMapFn<Input = I::Item>> {
    flat_map: F,
    input_list: I,
    output_list: Option<F::OutputList>,
}

impl<I: ListFn, F: FlatMapFn<Input = I::Item>> ListFn for FlatMapList<I, F> {
    type Item = <F::OutputList as ListFn>::Item;
    type End = I::End;
    fn next(mut self) -> ListState<Self> {
        loop {
            match self.output_list {
                Some(item_list) => match item_list.next() {
                    ListState::Some(ListSome { first, next }) => {
                        return ListState::Some(ListSome {
                            first,
                            next: FlatMapList {
                                flat_map: self.flat_map,
                                input_list: self.input_list,
                                output_list: Some(next),
                            },
                        })
                    }
                    ListState::End(..) => self.output_list = None,
                },
                None => match self.input_list.next() {
                    ListState::Some(ListSome { first, next }) => {
                        self.input_list = next;
                        self.output_list = Some(self.flat_map.map(first));
                    }
                    ListState::End(end) => return ListState::End(end),
                },
            }
        }
    }
}

pub trait FlatMap: ListFn {
    fn flat_map<F: FlatMapFn<Input = Self::Item>>(self, flat_map: F) -> FlatMapList<Self, F> {
        FlatMapList {
            input_list: self,
            flat_map,
            output_list: None,
        }
    }
}

impl<S: ListFn> FlatMap for S {}
