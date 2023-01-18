use super::*;

pub struct FilterScanWrap<S: FilterScanFn>(S);

pub type FilterScanState<S> = OptionList<<S as FilterScanFn>::OutputItem, FilterScanWrap<S>>;

pub trait FilterScanFn: Sized {
    type InputItem;
    type InputResult;
    type OutputItem;
    type OutputResult;
    fn map_input(self, input: Self::InputItem) -> FilterScanState<Self>;
    fn map_result(self, result: Self::InputResult) -> Self::OutputResult;
    fn some(self, first: Self::OutputItem) -> FilterScanState<Self> {
        OptionList::Some {
            first,
            end: FilterScanWrap(self),
        }
    }
    fn end(self) -> FilterScanState<Self> {
        OptionList::End(FilterScanWrap(self))
    }
}

impl<S: FilterScanFn> FlatScanFn for FilterScanWrap<S> {
    type InputItem = S::InputItem;
    type InputResult = S::InputResult;
    type OutputList = OptionList<S::OutputItem, Self>;
    type EndList = OptionList<S::OutputItem, Id<S::OutputResult>>;
    fn map_item(self, input: Self::InputItem) -> Self::OutputList {
        self.0.map_input(input)
    }
    fn map_result(self, result: Self::InputResult) -> Self::EndList {
        OptionList::End(Id::new(self.0.map_result(result)))
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

impl<L> FilterScan for L
where
    L: ListFn,
    L::End: ResultFn,
{
}
