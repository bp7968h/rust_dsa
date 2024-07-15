use rust_dsa::sorting::{bubble_sort, selection_sort, insertion_sort, quick_sort, counting_sort};

fn main() {
    let mut bubble_array = vec![4, 2, 5, 1, 3];
    bubble_sort(&mut bubble_array);
    println!("Bubble sorted: {:?}", bubble_array);

    let mut selection_array = vec![4, 2, 5, 1, 3];
    selection_sort(&mut selection_array);
    println!("Selection sorted: {:?}", selection_array);

    let mut insertion_array = vec![4, 2, 5, 1, 3];
    insertion_sort(&mut insertion_array);
    println!("Insertion sorted: {:?}", insertion_array);

    let mut quick_array = vec![4, 2, 5, 1, 3];
    let len = quick_array.len();
    quick_sort(&mut quick_array, 0, (len -1) as isize);
    println!("Quick sorted: {:?}", quick_array);

    let mut counting_array: [usize; 16] = [5, 3, 8, 6, 2, 7, 4, 3, 8, 7, 2, 5, 9, 0, 1, 4];
    counting_sort(&mut counting_array);
    println!("Counting sorted: {:?}", counting_array);

}
