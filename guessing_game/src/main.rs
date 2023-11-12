use std::io;

fn main() {
    println!("¡Adivina el número (Entre 1 y 100)!");

    println!("Por favor ingresa tu número");

    let mut guess = String::new();
    let secret_number = std::rand::thread_rng().gen_range(1, 101);

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea 💀");


    let mut guess_int: u32 = guess.trim().parse().expect("Por favor ingresa un número");

    while guess_int != secret_number {
        println!("¡Ese no es el número!");
        if guess_int > secret_number {
            println!("¡El número es menor!");
        } else {
            println!("¡El número es mayor!");
        }
        println!("Por favor ingresa tu número");
        if guess_int > 100 || guess_int < 1 {
            println!("El número debe estar entre 1 y 100");
        }
        io::stdin()
            .read_line(&mut guess)
            .expect("No se pudo leer la línea 💀");

        guess_int = guess.trim().parse().expect("Por favor ingresa un número");
    }

}
