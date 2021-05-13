use super::*;

/// A list.
pub enum ListState<F: ListFn> {
    /// The list has one item and a function to get a next sub list.
    Some(F::Item, F),
    /// The end of the list.
    End(F::End),
}

/// A function which returns a list.
pub trait ListFn: Sized {
    /// A list item type.
    type Item;
    /// A value which is returned when the list has no more items.
    type End;
    /// The main function which returns a list.
    fn state(self) -> ListState<Self>;
}
