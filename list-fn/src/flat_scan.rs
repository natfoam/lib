use super::*;

pub trait FlatScan {
    type InputItem;
    type ItemList: ListFn<End = Self>;
    type EndList: ListFn<Item = <Self::ItemList as ListFn>::Item>;
    fn item(self, input: Self::InputItem) -> Self::ItemList;
    fn end(self) -> Self::EndList;
}

pub enum FlatScanList<I: ListFn, F: FlatScan<InputItem = I::Item>> {
    Begin { flat_scan: F, input: I },
    Item { item_list: F::ItemList, input: I },
    End(F::EndList),
}

impl<I: ListFn, F: FlatScan<InputItem = I::Item>> ListFn for FlatScanList<I, F> {
    type Item = <F::ItemList as ListFn>::Item;
    type End = <F::EndList as ListFn>::End;
    fn list(mut self) -> List<Self> {
        loop {
            self = match self {
                FlatScanList::Begin { input, flat_scan } => match input.list() {
                    List::Some(first, next) => FlatScanList::Item {
                        item_list: flat_scan.item(first),
                        input: next,
                    },
                    List::End(..) => FlatScanList::End(flat_scan.end()),
                },
                FlatScanList::Item { item_list, input } => match item_list.list() {
                    List::Some(first, item_list) => {
                        return List::Some(first, FlatScanList::Item { item_list, input })
                    }
                    List::End(flat_scan) => FlatScanList::Begin { input, flat_scan },
                },
                FlatScanList::End(end_list) => {
                    return match end_list.list() {
                        List::Some(first, next) => List::Some(first, FlatScanList::End(next)),
                        List::End(end) => List::End(end),
                    }
                }
            }
        }
    }
}

pub trait FlatScanEx: ListFn {
    fn flat_scan<F: FlatScan<InputItem = Self::Item>>(self, flat_scan: F) -> FlatScanList<Self, F> {
        FlatScanList::Begin {
            input: self,
            flat_scan,
        }
    }
}

impl<S: ListFn> FlatScanEx for S {}
