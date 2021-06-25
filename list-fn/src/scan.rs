use super::*;

pub struct ScanState<S: ScanFn> {
    pub first: S::OutputItem,
    pub next: S,
}

pub trait ScanFn: Sized {
    type InputItem;
    type InputResult;
    type OutputItem;
    type OutputResult;
    fn map_input(self, input: Self::InputItem) -> ScanState<Self>;
    fn map_result(self, result: Self::InputResult) -> Self::OutputResult;
}

pub struct ScanWrap<S: ScanFn>(S);

impl<S: ScanFn> FlatScanFn for ScanWrap<S> {
    type InputItem = S::InputItem;
    type InputResult = S::InputResult;
    type OutputList = OptionList<S::OutputItem, Self>;
    type EndList = OptionList<S::OutputItem, Id<S::OutputResult>>;
    fn map_item(self, input: Self::InputItem) -> Self::OutputList {
        let ScanState { first, next } = self.0.map_input(input);
        OptionList::Some {
            first,
            end: ScanWrap(next),
        }
    }
    fn map_result(self, result: Self::InputResult) -> Self::EndList {
        OptionList::End(Id::new(self.0.map_result(result)))
    }
}

pub trait Scan
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn scan<S: ScanFn<InputItem = Self::Item, InputResult = <Self::End as ResultFn>::Result>>(
        self,
        scan: S,
    ) -> FlatScanState<Self, ScanWrap<S>> {
        self.flat_scan(ScanWrap(scan))
    }
}

impl<L> Scan for L
where
    Self: ListFn,
    Self::End: ResultFn,
{
}
