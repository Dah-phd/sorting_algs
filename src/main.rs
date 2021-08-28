struct BacktrackSort<'a> {
    vec: &'a mut Vec<f64>,
}
impl<'a> BacktrackSort<'a> {
    fn sorting_algorithm(&mut self, first_i: usize, second_i: usize) -> bool {
        if second_i == self.vec.len() {
            return true;
        }
        while self.vec[first_i] <= self.vec[second_i] {
            if self.sorting_algorithm(first_i + 1, second_i + 1) {
                return true;
            }
        }
        self.shuffle(first_i, second_i);
        false
    }
    fn shuffle(&mut self, first_i: usize, second_i: usize) {
        let save = self.vec[first_i];
        self.vec[first_i] = self.vec[second_i];
        self.vec[second_i] = save;
    }
    fn sort(mut sorted_vec: Vec<f64>) -> Vec<f64> {
        let mut sorter = BacktrackSort {
            vec: &mut sorted_vec,
        };
        while !sorter.sorting_algorithm(0, 1) {}
        sorted_vec
    }
}

fn main() {
    let mut vec: Vec<f64> = vec![5.0, 2.0, 3., 54., 5., 6., 7., 8.];
    let sorted = BacktrackSort::sort(vec);
    println!("{:?}", sorted)
}
