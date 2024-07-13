pub mod sorting;

#[cfg(test)]
mod tests{
    use crate::sorting::{bubble_sort, selection_sort, insertion_sort};

    #[test]
    fn test_bubble_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        bubble_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        selection_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}