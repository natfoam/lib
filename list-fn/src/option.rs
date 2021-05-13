use super::*;

/// Option as a list.
impl<T> ListFn for Option<T> {
    type Item = T;
    type End = Self;
    fn state(self) -> ListState<Self> {
        match self {
            Option::Some(first) => ListState::Some(first, Option::None),
            Option::None => ListState::End(Option::None),
        }
    }
}

impl<T> ResultFn for Option<T> {
    type Result = ();
    fn result(self) {}
}
