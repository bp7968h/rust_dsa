pub fn linear_search<T: Ord + Copy>(arr: &[T], value: T) -> Option<usize>{
    for (index, item) in arr.iter().enumerate(){
        if *item == value{
            return Some(index);
        }
    }
    None
}