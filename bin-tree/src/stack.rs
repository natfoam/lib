use crate::Node;

pub trait Stack {
    type Node: Node;
    type RevIterator: Iterator<Item = (Self::Node, u8)>;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: (Self::Node, u8));
    fn pop_if(&mut self, level: u8) -> Option<Self::Node>;
    fn rev_iter(self) -> Self::RevIterator;
}
