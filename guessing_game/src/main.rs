use std::io;

fn main() {
    println!("¡Adivina el número (Entre 1 y 100)!");

    println!("Por favor ingresa tu número");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea 💀");

    println!("Tu número es: {guess}");

    let guess_int: u32 = guess.trim().parse().expect("Por favor ingresa un número");
    if guess_int > 100 || guess_int < 1 {
        println!("El número debe estar entre 1 y 100");
        return;
    }
}
