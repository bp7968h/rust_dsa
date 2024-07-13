use rust_dsa::sorting::{bubble_sort, selection_sort, insertion_sort};

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

}
