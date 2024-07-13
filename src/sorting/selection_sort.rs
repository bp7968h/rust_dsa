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