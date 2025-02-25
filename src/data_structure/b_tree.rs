// B-Tree nodes have an array of elements with an array of children.
// For some constant/order B, each node contains between B-1 and 2B-1 elements in sorted order
// ***root can have as few as one element***
// An internal node (one which has children) with k elements has k+1 children.

#[derive(Debug, Default, PartialEq, Clone)]
struct Node {
    keys: Vec<usize>,
    childrens: Vec<Node>,
}

impl Node {
    fn key_len(&self) -> usize {
        self.keys.len()
    }

    fn child_len(&self) -> usize {
        self.childrens.len()
    }

    fn is_leaf(&self) -> bool {
        self.childrens.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub struct BTree {
    order: usize,
    root: Option<Node>,
}

impl BTree {
    fn new(order: usize) -> Self {
        BTree { order, root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&mut self, key: usize) {
        match self.root.take() {
            Some(root_node) if root_node.key_len() == self.order - 1 => {
                // Root is full, need to split first
                let mut new_root = Node::default();
                new_root.childrens.push(root_node);
                self.insert_non_full(key, &mut new_root);
                self.root = Some(new_root);
            }
            Some(mut root_node) => {
                self.insert_non_full(key, &mut root_node);
                self.root = Some(root_node);
            }
            None => {
                self.root = Some(Node {
                    keys: vec![key],
                    childrens: Vec::new(),
                });
            }
        }
    }

    fn insert_non_full(&mut self, key: usize, node: &mut Node) {
        let mut i = node.key_len();

        if node.is_leaf() {
            // Find position to insert key
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            node.keys.insert(i, key);
        } else {
            // Find child to recurse on
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }

            let child_idx = i;
            if node.childrens[child_idx].key_len() == self.order - 1 {
                self.split_child(child_idx, node);
                if key > node.keys[child_idx] {
                    i += 1;
                }
            }
            self.insert_non_full(key, &mut node.childrens[i]);
        }
    }

    fn split_child(&mut self, index: usize, parent: &mut Node) {
        let mut new_node = Node::default();
        // For order 3, when node has [1,2,3], we want:
        // - left node to have [1]
        // - median to be 2
        // - right node to have [3]
        let mid = (self.order - 1) / 2;
        let mut child = parent.childrens[index].clone();

        // Move keys to new node (everything after mid)
        new_node.keys = child.keys.drain(mid + 1..).collect();

        // Get the median key (at mid position)
        let median = child.keys.remove(mid);

        // Move children if they exist
        if !child.is_leaf() {
            new_node.childrens = child.childrens.drain(mid + 1..).collect();
        }

        // Update the nodes
        parent.childrens[index] = child;
        parent.childrens.insert(index + 1, new_node);
        parent.keys.insert(index, median);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_b_tree() {
        let bt = BTree::new(2);
        assert_eq!(
            bt,
            BTree {
                order: 2,
                root: None,
            }
        );
        assert_eq!(true, bt.is_empty());
    }

    #[test]
    fn insert_into_root_until_full() {
        let mut bt = BTree::new(3);
        assert_eq!(bt.is_empty(), true);

        bt.insert(1);
        println!("{:?}", bt);
        assert_eq!(bt.is_empty(), false);
        assert_eq!(
            bt,
            BTree {
                order: 3,
                root: Some(Node {
                    keys: vec![1],
                    childrens: Vec::new(),
                })
            }
        );

        bt.insert(2);
        assert_eq!(
            bt,
            BTree {
                order: 3,
                root: Some(Node {
                    keys: vec![1, 2],
                    childrens: Vec::new(),
                })
            }
        );
    }

    #[test]
    fn insert_and_split() {
        let mut bt = BTree::new(3);
        assert_eq!(bt.is_empty(), true);

        bt.insert(1);
        bt.insert(2);
        // for order 3, the max key is 2 so it should split with next insert
        bt.insert(3);
        bt.insert(4);
        bt.insert(5);
        bt.insert(6);
        bt.insert(7);

        assert_eq!(
            bt,
            BTree {
                order: 3,
                root: Some(Node {
                    keys: vec![4],
                    childrens: vec![
                        Node {
                            keys: vec![2],
                            childrens: vec![
                                Node {
                                    keys: vec![1],
                                    childrens: Vec::new(),
                                },
                                Node {
                                    keys: vec![3],
                                    childrens: Vec::new(),
                                }
                            ]
                        },
                        Node {
                            keys: vec![6],
                            childrens: vec![
                                Node {
                                    keys: vec![5],
                                    childrens: Vec::new(),
                                },
                                Node {
                                    keys: vec![7],
                                    childrens: Vec::new(),
                                }
                            ]
                        }
                    ]
                })
            }
        );
    }
}
