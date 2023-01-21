use crate::Node;

pub trait Stack: Iterator<Item = (Self::Node, u8)> {
    type Node: Node;
    fn with_capacity(i: &impl Iterator) -> Self;
    fn push(&mut self, value: (Self::Node, u8));
    fn pop_if(&mut self, level: u8) -> Option<Self::Node>;
}
