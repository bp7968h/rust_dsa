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

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: Some(self),
        }
    }

    pub fn add(&mut self, value: T) {
        let mut current = self;
        while let Some(ref mut next) = current.pointer {
            current = next;
        }
        current.pointer = Some(Box::new(LinkedList::new(value)));
    }

    pub fn find(&self, value: T) -> Option<&LinkedList<T>> {
        let mut current = Some(self);
        while let Some(ref node) = current {
            if node.data == value {
                return Some(node);
            }
            current = node.pointer.as_deref();
        }
        None
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn linked_list_test_add() {
        let mut list = LinkedList::new(1);
        list.add(2);
        list.add(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn linked_list_test_add_single() {
        let mut list = LinkedList::new(10);
        list.add(20);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn linked_list_test_add_multiple() {
        let mut list = LinkedList::new(1);
        for i in 2..=5 {
            list.add(i);
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_find_method_various_scenarios() {
        let mut list = LinkedList::new(1);
        list.add(2);
        list.add(3);

        // Find head
        assert_eq!(list.find(1).unwrap().data, 1, "Failed to find the head element.");

        // Find middle
        assert_eq!(list.find(2).unwrap().data, 2, "Failed to find the middle element.");

        // Find tail
        assert_eq!(list.find(3).unwrap().data, 3, "Failed to find the tail element.");

        //Trying to find a non-existent element
        assert!(list.find(4).is_none(), "Non-existent element was supposedly found.");
    }
}