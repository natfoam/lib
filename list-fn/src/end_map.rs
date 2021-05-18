use super::*;

pub trait EndMapFn {
    type Input;
    type Output;
    fn map(self, input: Self::Input) -> Self::Output;
}

pub struct EndMapList<I, E>
where
    I: ListFn,
    I::End: ResultFn,
    E: EndMapFn<Input = <I::End as ResultFn>::Result>,
{
    input: I,
    end_map: E,
}

impl<I, E> ListFn for EndMapList<I, E>
where
    I: ListFn,
    I::End: ResultFn,
    E: EndMapFn<Input = <I::End as ResultFn>::Result>,
{
    type Item = I::Item;
    type End = Id<E::Output>;
    fn next(self) -> ListState<Self> {
        match self.input.next() {
            ListState::Some(input, next) => ListState::Some(
                input,
                EndMapList {
                    input: next,
                    end_map: self.end_map,
                },
            ),
            ListState::End(end) => ListState::End(Id::new(self.end_map.map(end.result()))),
        }
    }
}

pub trait EndMap
where
    Self: ListFn,
    Self::End: ResultFn,
{
    fn end_map<E: EndMapFn<Input = <Self::End as ResultFn>::Result>>(
        self,
        end_map: E,
    ) -> EndMapList<Self, E> {
        EndMapList {
            input: self,
            end_map,
        }
    }
}

impl<T> EndMap for T
where
    T: ListFn,
    T::End: ResultFn,
{
}
