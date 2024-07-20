pub fn insertion_sort<T: Ord + std::fmt::Debug >(arr: &mut [T]){
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            j -= 1;
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}