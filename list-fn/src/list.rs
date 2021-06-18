pub struct ListSome<F: ListFn> {
    pub first: F::Item,
    pub next: F,
}

/// A list.
pub enum ListState<F: ListFn> {
    Some(ListSome<F>),
    /// The end of the list.
    End(F::End),
}

impl<F: ListFn> ListState<F> {
    pub fn some(first: F::Item, next: F) -> Self {
        ListState::Some(ListSome { first, next })
    }
}

/// A function which returns a list.
pub trait ListFn: Sized {
    /// A list item type.
    type Item;
    /// A value which is returned when the list has no more items.
    type End;
    /// The main function which returns a list.
    fn next(self) -> ListState<Self>;
}
