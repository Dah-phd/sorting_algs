pub fn bakctracking_sort<T: Into<f64> + Copy>(vec: &Vec<T>) -> Vec<T> {
    let mut vec = vec.to_vec();
    while !sorting_algorithm(&mut vec, 0, 1) {}
    vec
}
pub fn bakctracking_sort_inplace<T: Into<f64> + Copy>(vec: &mut Vec<T>) {
    while !sorting_algorithm(vec, 0, 1) {}
}

fn sorting_algorithm<T: Into<f64> + Copy>(
    vector: &mut Vec<T>,
    first_i: usize,
    second_i: usize,
) -> bool {
    if second_i == vector.len() {
        return true;
    }
    while vector[first_i].into() <= vector[second_i].into() {
        if sorting_algorithm(vector, first_i + 1, second_i + 1) {
            return true;
        }
    }
    move_values(vector, first_i, second_i);
    false
}
fn move_values<T: Into<f64> + Copy>(vector: &mut Vec<T>, first_i: usize, second_i: usize) {
    let save = vector[first_i];
    vector[first_i] = vector[second_i];
    vector[second_i] = save;
}
