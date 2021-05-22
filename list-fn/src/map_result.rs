use super::*;

pub trait MapResultFn {
    type Input;
    type Output;
    fn map(self, input: Self::Input) -> Self::Output;
}

pub struct MapResultList<I, E>
where
    I: ListFn,
    I::End: ResultFn,
    E: MapResultFn<Input = <I::End as ResultFn>::Result>,
{
    input: I,
    map: E,
}

impl<I, E> ListFn for MapResultList<I, E>
where
    I: ListFn,
    I::End: ResultFn,
    E: MapResultFn<Input = <I::End as ResultFn>::Result>,
{
    type Item = I::Item;
    type End = Id<E::Output>;
    fn next(self) -> ListState<Self> {
        match self.input.next() {
            ListState::Some(ListSome { first, next }) => ListState::Some(ListSome {
                first,
                next: MapResultList {
                    input: next,
                    map: self.map,
                },
            }),
            ListState::End(end) => ListState::End(Id::new(self.map.map(end.result()))),
        }
    }
}

pub trait MapResult
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn map_result<E: MapResultFn<Input = <Self::End as ResultFn>::Result>>(
        self,
        map: E,
    ) -> MapResultList<Self, E> {
        MapResultList { input: self, map }
    }
}

impl<T> MapResult for T
where
    T: ListFn,
    T::End: ResultFn,
{
}
