pub mod sorting;

#[cfg(test)]
mod tests{
    use crate::sorting::{bubble_sort, selection_sort, insertion_sort, quick_sort, counting_sort, radix_sort};

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

    #[test]
    fn test_quick_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        let len = array.len();
        quick_sort(&mut array, 0, (len - 1) as isize);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counting_sort() {
        let mut array = vec![4, 2, 5, 2, 1, 1, 0, 3];
        counting_sort(&mut array);
        assert_eq!(array, vec![0, 1, 1, 2, 2, 3, 4, 5]);
    }

    #[test]
    fn test_radix_sort(){
        let mut array = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut array);
        assert_eq!(array, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }
}