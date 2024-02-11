// use std::vec::Vec; // no lo necesitamos porque estamos usando la macro vec![]

fn main() {
    println!("Testing collections");
    let my_vector = vec![1, 2, 3];

    // Esto hace que el vector sea movido y no se puede usar después de terminar el for
    //for i in my_vector {
    //    println!("Value: {}", i + 1);
    // }

    // Para evitarlo, tenemos que usar una referencia
    for i in &my_vector {
        println!("Value: {:?}", &i);
        let summer = 4;
        let a = i + summer;
        println!("Value summed: {}", a);
    }
    // Y así podemos seguir usando el vector como si nada hubiera pasado
    println!("Vector: {:?}", my_vector[0]);
}
