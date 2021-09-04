//ALGORITHMS
//================================================
// algorithm using backtracking to sort Vec, by checking condition between two values and moving on
fn sorting_backtrack<T: Into<f64> + Copy>(
    vector: &mut Vec<T>,
    first_i: usize,
    second_i: usize,
) -> bool {
    if second_i == vector.len() {
        return true;
    }
    while vector[first_i].into() <= vector[second_i].into() {
        if sorting_backtrack(vector, first_i + 1, second_i + 1) {
            return true;
        }
    }
    // support function
    move_values(vector, first_i, second_i);
    false
}

// algorithm using recursion to sort Vec, by checking condition between two values and moving on
fn sorting_recursive<T: Into<f64> + Copy>(vector: &mut Vec<T>, first_i: usize, second_i: usize) {
    if vector[first_i].into() <= vector[second_i].into() {
        if second_i < vector.len() - 1 {
            sorting_recursive(vector, first_i + 1, second_i + 1)
        }
    } else {
        // support functio
        move_values(vector, first_i, second_i);
        if first_i != 0 {
            sorting_recursive(vector, first_i - 1, second_i - 1)
        } else {
            sorting_recursive(vector, first_i + 1, second_i + 1)
        }
    }
}

// TRIGGER FUNCTION
//================================================
pub fn bakctracking_sort<T: Into<f64> + Copy>(vec: &Vec<T>) -> Vec<T> {
    let mut vec = vec.to_vec();
    while !sorting_backtrack(&mut vec, 0, 1) {}
    vec
}

pub fn bakctracking_sort_inplace<T: Into<f64> + Copy>(vec: &mut Vec<T>) {
    while !sorting_backtrack(vec, 0, 1) {}
}

pub fn recursive_sort<T: Into<f64> + Copy>(vec: &Vec<T>) -> Vec<T> {
    let mut vec = vec.to_vec();
    sorting_recursive(&mut vec, 0, 1);
    vec
}

pub fn recursive_sort_inplace<T: Into<f64> + Copy>(vec: &mut Vec<T>) {
    sorting_recursive(vec, 0, 1);
}

//SUPPORT FUNCTIONS
//================================================

fn move_values<T: Into<f64> + Copy>(vector: &mut Vec<T>, first_i: usize, second_i: usize) {
    let save = vector[first_i];
    vector[first_i] = vector[second_i];
    vector[second_i] = save;
}
