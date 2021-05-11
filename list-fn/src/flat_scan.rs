use super::*;

pub trait FlatScan {
    type InputItem;
    type InputResult;
    type ItemList: ListFn<End = Self>;
    type EndList: ListFn<Item = <Self::ItemList as ListFn>::Item>;
    fn item(self, input: Self::InputItem) -> Self::ItemList;
    fn end(self, result: Self::InputResult) -> Self::EndList;
}

pub enum FlatScanListFn<I: ListFn, F: FlatScan<InputItem = I::Item>>
where
    I::End: ResultFn<Result = F::InputResult>,
{
    Begin { flat_scan: F, input: I },
    ItemList { item_list: F::ItemList, input: I },
    EndList(F::EndList),
    End(<F::EndList as ListFn>::End),
}

impl<I: ListFn, F: FlatScan<InputItem = I::Item>> ListFn for FlatScanListFn<I, F>
where
    I::End: ResultFn<Result = F::InputResult>,
{
    type Item = <F::ItemList as ListFn>::Item;
    type End = <F::EndList as ListFn>::End;
    fn list(mut self) -> List<Self> {
        loop {
            self = match self {
                FlatScanListFn::Begin { input, flat_scan } => match input.list() {
                    List::Some(first, next) => FlatScanListFn::ItemList {
                        item_list: flat_scan.item(first),
                        input: next,
                    },
                    List::End(end) => FlatScanListFn::EndList(flat_scan.end(end.result())),
                },
                FlatScanListFn::ItemList { item_list, input } => match item_list.list() {
                    List::Some(first, item_list) => {
                        return List::Some(first, FlatScanListFn::ItemList { item_list, input })
                    }
                    List::End(flat_scan) => FlatScanListFn::Begin { input, flat_scan },
                },
                FlatScanListFn::EndList(end_list) => {
                    return match end_list.list() {
                        List::Some(first, next) => List::Some(first, FlatScanListFn::EndList(next)),
                        List::End(end) => List::End(end),
                    }
                }
                FlatScanListFn::End(end) => FlatScanListFn::End(end),
            }
        }
    }
}

pub trait FlatScanEx: ListFn {
    fn flat_scan<F: FlatScan<InputItem = Self::Item>>(self, flat_scan: F) -> FlatScanListFn<Self, F>
    where
        Self::End: ResultFn<Result = F::InputResult>,
    {
        FlatScanListFn::Begin {
            input: self,
            flat_scan,
        }
    }
}

impl<S: ListFn> FlatScanEx for S {}
