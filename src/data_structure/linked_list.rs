use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct LinkedList<T: Ord> {
    pub data: T,
    pub pointer: Option<Box<LinkedList<T>>>,
}

impl<T: Ord> LinkedList<T> {
    pub fn new(value: T) -> Self {
        LinkedList {
            data: value,
            pointer: None,
        }
    }

    pub fn add(&mut self, value: T) {
        let mut current = self;
        while let Some(ref mut next) = current.pointer {
            current = next;
        }
        current.pointer = Some(Box::new(LinkedList::new(value)));
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: Some(self),
        }
    }
}

pub struct LinkedListIter<'a, T: Ord> {
    current: Option<&'a LinkedList<T>>,
}

impl<'a, T: Ord> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current {
            self.current = node.pointer.as_deref();
            Some(&node.data)
        } else {
            None
        }
    }
}

impl<T: Ord + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self;
        write!(f, "{}", current.data)?;

        while let Some(ref next) = current.pointer {
            write!(f, " -> {}", next.data)?;
            current = next;
        }

        write!(f, " -> None")?;

        Ok(())
    }
}