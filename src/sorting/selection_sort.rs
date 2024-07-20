pub fn selection_sort<T: Ord>(arr: &mut [T]){
    let len = arr.len();
    for i in 0..len {
        let mut low_idx = i;
        for j in i+1..len {
            if arr[low_idx] > arr[j]{
                low_idx = j
            }
        }
        if i != low_idx {
            arr.swap(i,low_idx);
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        selection_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}