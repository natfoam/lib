use super::*;

impl<T> ListFn for Option<T> {
    type Item = T;
    type End = Self;
    fn list(self) -> List<Self> {
        match self {
            Option::Some(first) => List::Some(first, Option::None),
            Option::None => List::End(Option::None),
        }
    }
}
