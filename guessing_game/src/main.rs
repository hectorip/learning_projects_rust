use rand::Rng;
use std::io;

// Ejercicio de Juego de adivinar el número
// Temas que incluye:
//  - Importación de crates y de biblioteca estándar
//  - Manejo de errores
//  - Mutabilidad y declaración de variables
//  - Entrada y salida de datos
//  - Generación de números aleatorios
//  - Bucles
//  - Conversión de tipos

fn main() {
    println!("¡Adivina el número (Entre 1 y 100)!");

    println!("Por favor ingresa tu número");

    let mut guess = String::new();
    let mut rng = rand::thread_rng();
    let secret_number  = rng.gen_range(1..101);

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea 💀");

        let mut guess_int: u32 = guess.trim().parse().expect("Error al convertir el número: ");

    while guess_int != secret_number {

        if guess_int > secret_number {
            println!("¡El número es menor!");
        } else {
            println!("¡El número es mayor!");
        }
        println!("Por favor ingresa tu número");
        if guess_int > 100 || guess_int < 1 {
            println!("El número debe estar entre 1 y 100");
        }
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("No se pudo leer la línea 💀");

        guess_int = guess.trim().parse().expect("Error al convertir el número: ");
    }

    println!("¡Ganaste! 🎉 el número es {secret_number}");
}
