use super::*;

pub trait FlatScan {
    type InputItem;
    type ItemList: ListFn<End = Self>;
    type EndList: ListFn<Item = <Self::ItemList as ListFn>::Item>;
    fn item(self, input: Self::InputItem) -> Self::ItemList;
    fn end(self) -> Self::EndList;
}

pub enum FlatScanListFn<I: ListFn, F: FlatScan<InputItem = I::Item>> {
    Begin { flat_scan: F, input: I },
    ItemList { item_list: F::ItemList, input: I },
    EndList(F::EndList),
    End(<F::EndList as ListFn>::End),
}

impl<I: ListFn, F: FlatScan<InputItem = I::Item>> ListFn for FlatScanListFn<I, F> {
    type Item = <F::ItemList as ListFn>::Item;
    type End = Self;
    fn list(mut self) -> List<Self> {
        loop {
            self = match self {
                FlatScanListFn::Begin { input, flat_scan } => match input.list() {
                    List::Some(first, next) => FlatScanListFn::ItemList {
                        item_list: flat_scan.item(first),
                        input: next,
                    },
                    List::End(..) => FlatScanListFn::EndList(flat_scan.end()),
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
                        List::End(end) => List::End(FlatScanListFn::End(end)),
                    }
                }
                FlatScanListFn::End(end) => FlatScanListFn::End(end),
            }
        }
    }
}

pub trait FlatScanEx: ListFn {
    fn flat_scan<F: FlatScan<InputItem = Self::Item>>(
        self,
        flat_scan: F,
    ) -> FlatScanListFn<Self, F> {
        FlatScanListFn::Begin {
            input: self,
            flat_scan,
        }
    }
}

impl<S: ListFn> FlatScanEx for S {}
