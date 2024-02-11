use std::vec::Vec;

fn creating_a_vector(data: i32) -> Vec<i32> {
    // Tratando de crear un vector sin usar la macro vec![]
    let the_vector = Vec::from(data);
    the_vector
}
