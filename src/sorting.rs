pub struct BacktrackSort<'a, T: Into<f64>> {
    vec: &'a mut Vec<T>,
}
impl<'a, T: Into<f64> + Copy> BacktrackSort<'a, T> {
    fn sorting_algorithm(&mut self, first_i: usize, second_i: usize) -> bool {
        if second_i == self.vec.len() {
            return true;
        }
        while self.vec[first_i].into() <= self.vec[second_i].into() {
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
    pub fn sort(mut sorted_vec: Vec<T>) -> Vec<T> {
        let mut sorter = BacktrackSort {
            vec: &mut sorted_vec,
        };
        while !sorter.sorting_algorithm(0, 1) {}
        sorted_vec
    }
    // fn parallel_shuffle(&mut self, first_i: usize, second_i: usize) {
    //     let save = self.vec[first_i];
    //     self.vec[first_i] = self.vec[second_i];
    //     self.vec[second_i] = save;
    //     for vector in self.parallel_vecs {
    //         vector
    //     }
    // }

    pub fn sort_parallel(mut sorted_vec: Vec<T>) -> Vec<T> {
        sorted_vec
    }
}
