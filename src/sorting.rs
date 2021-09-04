use std::collections::HashMap;

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

// point by all comperison with recursion
fn onetoall_recursive<T: Into<f64> + Copy>(
    vector: &Vec<T>,
    position: usize,
    positions: &mut HashMap<usize, usize>,
) {
    let mut new_position: usize = 0;
    for i in 0..vector.len() {
        if vector[i].into() < vector[position].into() {
            new_position += 1;
        }
    }
    positions.insert(new_position, position);
    if position < vector.len() - 1 {
        onetoall_recursive(vector, position + 1, positions)
    }
}

// TRIGGER FUNCTION
//================================================
pub fn onetoallrec_sort<T: Into<f64> + Copy>(vec: &Vec<T>) -> (Vec<T>, HashMap<usize, usize>) {
    let mut hash = HashMap::new();
    onetoall_recursive(vec, 0, &mut hash);
    let mut vector = Vec::new();
    for i in 0..vec.len() {
        let val = hash.get(&i);
        if val.is_none() {
            vector.push(vector[i - 1])
        } else {
            vector.push(vec[*val.unwrap()])
        }
    }
    (vector, hash)
}

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
