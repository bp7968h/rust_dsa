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