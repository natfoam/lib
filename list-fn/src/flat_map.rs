use super::*;

pub trait FlatMapFn {
    type InputItem;
    type ItemList: ListFn;
    /// Map the given `input` item into a list.
    fn map(&self, input: Self::InputItem) -> Self::ItemList;
}

pub struct FlatMapList<I: ListFn, F: FlatMapFn<InputItem = I::Item>> {
    flat_map: F,
    input_list: I,
    item_list: Option<F::ItemList>,
}

impl<I: ListFn, F: FlatMapFn<InputItem = I::Item>> ListFn for FlatMapList<I, F> {
    type Item = <F::ItemList as ListFn>::Item;
    type End = I::End;
    fn next(mut self) -> ListState<Self> {
        loop {
            match self.item_list {
                Some(item_list) => match item_list.next() {
                    ListState::Some(item, next_item_list) => {
                        return ListState::Some(
                            item,
                            FlatMapList {
                                flat_map: self.flat_map,
                                input_list: self.input_list,
                                item_list: Some(next_item_list),
                            },
                        )
                    }
                    ListState::End(..) => self.item_list = None,
                },
                None => match self.input_list.next() {
                    ListState::Some(input, next_input_list) => {
                        self.input_list = next_input_list;
                        self.item_list = Some(self.flat_map.map(input));
                    }
                    ListState::End(end) => return ListState::End(end),
                },
            }
        }
    }
}

pub trait FlatMap: ListFn {
    fn flat_map<F: FlatMapFn<InputItem = Self::Item>>(self, flat_map: F) -> FlatMapList<Self, F> {
        FlatMapList {
            input_list: self,
            flat_map,
            item_list: None,
        }
    }
}

impl<S: ListFn> FlatMap for S {}