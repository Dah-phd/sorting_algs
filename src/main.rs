mod sorting;

fn main() {
    let mut vec: Vec<f64> = vec![5.0, 2.0, 3., 54., 5., 6., 7., 8.];
    println!("Original: {:?}", vec);
    println!("Using backtracking sort:");
    let sorted_by_backtrack = sorting::bakctracking_sort(&vec);
    println!("{:?}", sorted_by_backtrack);
    println!("Using recursive sort:");
    let sorted_by_rec = sorting::recursive_sort(&vec);
    println!("{:?}", sorted_by_rec);
    println!("Using onetoall recursive sort:");
    let sorted_by_onetoall = sorting::onetoallrec_sort(&vec);
    println!("{:?}", sorted_by_onetoall.0);
}
