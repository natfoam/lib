use super::*;

pub trait FlatScanFn {
    type InputItem;
    type InputResult;
    type ItemList: ListFn<End = Self>;
    type EndList: ListFn<Item = <Self::ItemList as ListFn>::Item>;
    /// Map the given `input` item into a list.
    fn map_item(self, input: Self::InputItem) -> Self::ItemList;
    /// Map the given `result` into an end list.
    fn map_result(self, result: Self::InputResult) -> Self::EndList;
}

pub enum FlatScanState<I, F>
where
    I: ListFn,
    F: FlatScanFn<InputItem = I::Item>,
    I::End: ResultFn<Result = F::InputResult>,
{
    Begin { flat_scan: F, input_list: I },
    ItemList { item_list: F::ItemList, input_list: I },
    EndList(F::EndList),
}

impl<I, F> ListFn for FlatScanState<I, F>
where
    I: ListFn,
    F: FlatScanFn<InputItem = I::Item>,
    I::End: ResultFn<Result = F::InputResult>,
{
    type Item = <F::ItemList as ListFn>::Item;
    type End = <F::EndList as ListFn>::End;
    fn next(mut self) -> ListState<Self> {
        loop {
            self = match self {
                FlatScanState::Begin { input_list, flat_scan } => match input_list.next() {
                    ListState::Some(first, next) => FlatScanState::ItemList {
                        item_list: flat_scan.map_item(first),
                        input_list: next,
                    },
                    ListState::End(end) => FlatScanState::EndList(flat_scan.map_result(end.result())),
                },
                FlatScanState::ItemList { item_list, input_list } => match item_list.next() {
                    ListState::Some(first, item_list) => {
                        return ListState::Some(first, FlatScanState::ItemList { item_list, input_list })
                    }
                    ListState::End(flat_scan) => FlatScanState::Begin { input_list, flat_scan },
                },
                FlatScanState::EndList(end_list) => {
                    return match end_list.next() {
                        ListState::Some(first, next) => {
                            ListState::Some(first, FlatScanState::EndList(next))
                        }
                        ListState::End(end) => ListState::End(end),
                    }
                }
            }
        }
    }
}

pub trait FlatScan: ListFn {
    fn flat_scan<F>(self, flat_scan: F) -> FlatScanState<Self, F>
    where
        F: FlatScanFn<InputItem = Self::Item>,
        Self::End: ResultFn<Result = F::InputResult>,
    {
        FlatScanState::Begin {
            input_list: self,
            flat_scan,
        }
    }
}

impl<S: ListFn> FlatScan for S {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFlatScan();

    impl ResultFn for TestFlatScan {
        type Result = ();
        fn result(self) {}
    }

    impl FlatScanFn for TestFlatScan {
        type InputItem = ();
        type InputResult = ();
        type ItemList = Empty<(), Self>;
        type EndList = Empty<(), Self>;
        fn map_item(self, _: ()) -> Self::ItemList {
            Empty::new(self)
        }
        fn map_result(self, _: ()) -> Self::EndList {
            Empty::new(self)
        }
    }

    #[test]
    fn flat_scan_end() {
        let x = Empty::<(), ()>::new(());
        let f = TestFlatScan();
        let list = x.flat_scan(f);
        let list1 = list.next();
        // let list2 = list1.state();
    }
}