// B-Tree nodes have an array of elements with an array of children.
// For some constant/order B, each node contains between B-1 and 2B-1 elements in sorted order
// ***root can have as few as one element***
// An internal node (one which has children) with k elements has k+1 children. 

#[derive(Debug, Default)]
struct Node {
    keys: Vec<usize>,
    childrens: Vec<Box<Node>>,
}

impl Node {
    fn key_len(&self) -> usize {
        self.keys.len()
    } 
}

#[derive(Debug)]
pub struct BTree {
    order: usize,
    root: Option<Box<Node>>,
}

impl BTree {
    fn new(order: usize) -> Self {
        BTree {
            order,
            root: None,
        }
    }
    
    fn insert(&mut self, data: usize) {
        if let Some(ref mut root_node) = self.root {
            todo!();
        } else {
            self.root = Some(Box::new(
                Node {
                    keys: vec![data],
                    ..Node::default()
                }
            ))
        }
    } 
}