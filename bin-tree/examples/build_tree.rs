use bin_tree::{Node, IteratorEx};

#[derive(Debug)]
struct NodeContent {
    value: usize,
    children: Option<(NodeRef, NodeRef)>,
}

type NodeRef = Box<NodeContent>;

fn leaf(value: usize) -> NodeRef {
    Box::new(NodeContent {
        value,
        children: None,
    })
}

impl Node for NodeRef {
    fn new_parent(self, right: Self) -> Self {
        let value = self.value + right.value;
        Box::new(NodeContent {
            value,
            children: Some((self, right)),
        })
    }

    fn new_parent_from_single(self) -> Self {
        self
    }
}

fn main() {
    let root = (0..11).map(leaf).build_tree().unwrap();
    assert_eq!(55, root.value);
    println!("root: {:?}", root);
    println!("root.children: {:?}", root.children);
}