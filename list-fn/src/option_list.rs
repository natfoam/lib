use super::*;

pub enum OptionList<I, E> {
    Some { item: I, end: E },
    End(E),
}

impl<I, E> ListFn for OptionList<I, E> {
    type Item = I;
    type End = E;
    fn next(self) -> ListState<Self> {
        match self {
            OptionList::Some { item, end } => ListState::Some(item, OptionList::End(end)),
            OptionList::End(end) => ListState::End(end),
        }
    }
}
