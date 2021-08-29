mod sorting;

fn main() {
    let vec: Vec<f64> = vec![5.0, 2.0, 3., 54., 5., 6., 7., 8.];
    let sorted = sorting::bakctracking_sort(&vec);
    println!("{:?}", sorted);
    println!("Original: {:?}", vec)
}
