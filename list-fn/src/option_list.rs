use super::*;

pub enum OptionList<I, E> {
    Some { first: I, end: E },
    End(E),
}

impl<I, E> ListFn for OptionList<I, E> {
    type Item = I;
    type End = E;
    fn next(self) -> ListState<Self> {
        match self {
            OptionList::Some { first, end } => ListState::Some(ListSome {
                first,
                next: OptionList::End(end),
            }),
            OptionList::End(end) => ListState::End(end),
        }
    }
}
