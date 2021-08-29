mod sorting;

fn main() {
    let mut vec: Vec<f64> = vec![5.0, 2.0, 3., 54., 5., 6., 7., 8.];
    let sorted = sorting::BacktrackSort::sort(vec);
    println!("{:?}", sorted);
}
