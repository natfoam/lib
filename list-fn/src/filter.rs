use super::*;

pub trait FilterFn {
    type Input;
    type Output;
    /// Map the given `input` item into an output item.
    fn map(&self, input: Self::Input) -> Option<Self::Output>;
}

pub struct FilterWrap<F: FilterFn>(pub F);

impl<F: FilterFn> FlatMapFn for FilterWrap<F> {
    type Input = F::Input;
    type OutputList = Option<F::Output>;
    fn map(&self, input: Self::Input) -> Self::OutputList {
        self.0.map(input)
    }
}

pub trait Filter: ListFn {
    fn filter<F: FilterFn<Input = Self::Item>>(self, f: F) -> FlatMapList<Self, FilterWrap<F>> {
        self.flat_map(FilterWrap(f))
    }
}