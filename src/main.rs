use rust_dsa::sorting::{bubble_sort, selection_sort};

fn main() {
    let mut bubble_array = vec![4, 2, 5, 1, 3];
    bubble_sort(&mut bubble_array);
    println!("Bubble sorted: {:?}", bubble_array);

    let mut selection_array = vec![4, 2, 5, 1, 3];
    selection_sort(&mut selection_array);
    println!("Selection sorted: {:?}", selection_array);

}
