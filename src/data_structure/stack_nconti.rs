struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct StackNonConti<T>{
    top: Option<Box<Node<T>>>
}

impl<T> StackNonConti<T>{
    pub fn new() -> Self {
        StackNonConti{ top: None }
    }

    pub fn push(&mut self, data: T){
        let new_node = Box::new(Node {
            data,
            next: self.top.take(),
        });

        self.top = Some(new_node)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn stack_nconti_add(){
        let mut stack = StackNonConti::new();

        assert!(stack.top.is_none());

        stack.push(1);
        stack.push(2);

        assert_eq!(stack.top.as_ref().map(|node| node.data), Some(2));

        stack.push(3);

        assert_eq!(stack.top.as_ref().map(|node| node.data), Some(3));
    }
}