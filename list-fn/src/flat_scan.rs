use super::*;

pub trait FlatScanFn {
    type InputItem;
    type InputResult;
    type ItemList: ListFn<End = Self>;
    type EndList: ListFn<Item = <Self::ItemList as ListFn>::Item>;
    fn item(self, input: Self::InputItem) -> Self::ItemList;
    fn end(self, result: Self::InputResult) -> Self::EndList;
}

pub enum FlatScanState<I, F>
where
    I: ListFn,
    F: FlatScanFn<InputItem = I::Item>,
    I::End: ResultFn<Result = F::InputResult>,
{
    Begin { flat_scan: F, input: I },
    ItemList { item_list: F::ItemList, input: I },
    EndList(F::EndList),
    End(<F::EndList as ListFn>::End),
}

impl<I, F> ListFn for FlatScanState<I, F>
where
    I: ListFn,
    F: FlatScanFn<InputItem = I::Item>,
    I::End: ResultFn<Result = F::InputResult>,
{
    type Item = <F::ItemList as ListFn>::Item;
    type End = <F::EndList as ListFn>::End;
    fn state(mut self) -> ListState<Self> {
        loop {
            self = match self {
                FlatScanState::Begin { input, flat_scan } => match input.state() {
                    ListState::Some(first, next) => FlatScanState::ItemList {
                        item_list: flat_scan.item(first),
                        input: next,
                    },
                    ListState::End(end) => FlatScanState::EndList(flat_scan.end(end.result())),
                },
                FlatScanState::ItemList { item_list, input } => match item_list.state() {
                    ListState::Some(first, item_list) => {
                        return ListState::Some(first, FlatScanState::ItemList { item_list, input })
                    }
                    ListState::End(flat_scan) => FlatScanState::Begin { input, flat_scan },
                },
                FlatScanState::EndList(end_list) => {
                    return match end_list.state() {
                        ListState::Some(first, next) => {
                            ListState::Some(first, FlatScanState::EndList(next))
                        }
                        ListState::End(end) => ListState::End(end),
                    }
                }
                end => end,
            }
        }
    }
}

pub trait FlatScan: ListFn {
    fn flat_scan<F: FlatScanFn<InputItem = Self::Item>>(
        self,
        flat_scan: F,
    ) -> FlatScanState<Self, F>
    where
        Self::End: ResultFn<Result = F::InputResult>,
    {
        FlatScanState::Begin {
            input: self,
            flat_scan,
        }
    }
}

impl<S: ListFn> FlatScan for S {}
