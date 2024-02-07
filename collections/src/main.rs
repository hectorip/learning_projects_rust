use std::vec::Vec;

fn main() {
    println!("Testing collections");
    let mut my_vector = vec![1, 2, 3];

    // Esto hace que el vector sea movido y no se puede usar después de terminar el for
    //for i in my_vector {
    //    println!("Value: {}", i + 1);
    // }

    // Para evitarlo, tenemos que usar una referencia
    for i in &my_vector {
        println!("Value: {}", *i + 1);
    }

    println!("Vector: {:?}", my_vector);
}
