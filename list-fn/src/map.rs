use super::*;

pub trait MapFn {
    type Input;
    type Output;
    /// Map the given `input` item into an output item.
    fn map(&self, input: Self::Input) -> Self::Output;
}

pub struct MapWrap<M: MapFn>(M);

impl<M: MapFn> FlatMapFn for MapWrap<M> {
    type Input = M::Input;
    type OutputList = Option<M::Output>;
    fn map(&self, input: Self::Input) -> Self::OutputList {
        Some(self.0.map(input))
    }
}

pub trait Map: ListFn {
    fn map<M: MapFn<Input = Self::Item>>(self, m: M) -> FlatMapList<Self, MapWrap<M>> {
        self.flat_map(MapWrap(m))
    }
}
