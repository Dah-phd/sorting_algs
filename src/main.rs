mod sorting;

fn main() {
    println!("Using backtracking sort:");
    let mut vec: Vec<f64> = vec![5.0, 2.0, 3., 54., 5., 6., 7., 8.];
    let sorted = sorting::bakctracking_sort(&vec);
    println!("{:?}", sorted);
    println!("Original: {:?}", vec);
    println!("Using backtracking sort inplace:");
    sorting::bakctracking_sort_inplace(&mut vec);
    println!("Original: {:?}", vec);
}
