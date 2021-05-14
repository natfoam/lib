use super::*;

pub struct FilterScanState<S: FilterScanFn> {
    item: Option<S::OutputItem>,
    next: S,
}

pub trait FilterScanFn: Sized {
    type InputItem;
    type InputResult;
    type OutputItem;
    type OutputResult;
    fn map_input(self, input: Self::InputItem) -> FilterScanState<Self>;
    fn map_result(self, result: Self::InputResult) -> Self::OutputResult;
}

pub struct FilterScanWrap<S: FilterScanFn>(S);

impl<S: FilterScanFn> FlatScanFn for FilterScanWrap<S> {
    type InputItem = S::InputItem;
    type InputResult = S::InputResult;
    type OutputList = OptionList<S::OutputItem, Self>;
    type EndList = OptionList<S::OutputItem, S::OutputResult>;
    fn map_item(self, input: Self::InputItem) -> Self::OutputList {
        let FilterScanState { item, next } = self.0.map_input(input);
        let end = FilterScanWrap(next);
        match item {
            Some(item) => OptionList::Some { item, end },
            None => OptionList::End(end),
        }
    }
    fn map_result(self, result: Self::InputResult) -> Self::EndList {
        OptionList::End(self.0.map_result(result))
    }
}

pub trait FilterScan
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn filter_scan<
        S: FilterScanFn<InputItem = Self::Item, InputResult = <Self::End as ResultFn>::Result>,
    >(
        self,
        scan: S,
    ) -> FlatScanState<Self, FilterScanWrap<S>> {
        self.flat_scan(FilterScanWrap(scan))
    }
}
