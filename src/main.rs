mod sorting;

fn main() {
    println!("Using backtracking sort:");
    let mut vec: Vec<f64> = vec![5.0, 2.0, 3., 54., 5., 6., 7., 8.];
    let sorted_by_backtrack = sorting::bakctracking_sort(&vec);
    println!("{:?}", sorted_by_backtrack);
    println!("Using recursive sort:");
    let sorted_by_rec = sorting::recursive_sort(&vec);
    println!("{:?}", sorted_by_rec);
    println!("Original: {:?}", vec);
    let mut test_vec = vec![3, 2, 1, 2];
    test_vec[6] = 3;
    // println!("Using backtracking sort inplace:");
    // sorting::bakctracking_sort_inplace(&mut vec);
    // println!("Original: {:?}", vec);
}
