use rand::Rng;
use std::io;
use std::cmp::Ordering;
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
    guessing_game();
    println!("¡Adivina el número (1-100)!");

    println!("Por favor ingresa tu número: ");

    let mut guess = String::new();
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea 💀");

    let mut guess_int: u32 = guess
        .trim()
        .parse()
        .expect("Error al convertir el número: ");

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

        guess_int = guess
            .trim()
            .parse()
            .expect("Error al convertir el número: ");
    }

    println!("¡Ganaste! 🎉 el número es {secret_number}");
}

// La siguiente implementación es una solución más corta y elegante, además más adaptada a Rust en general:

fn guessing_game() {
    // Instrucciones
    println!("Bienvenido al juego de la adivinanza: voy a elegir un número y tú tendrás que adivinarlo. El número está entre 1 y 100.");
    // Generar número aleatorio
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Número secreto: {secret_number}");
    let mut guess = String.new(); 
    io::stdin()
        .read_line(&mut guess)
        .expect("Error fatal: no se pudo leer la entrada.")

    let guess = match guess.trim().parse() {
        OK(n) => n,
        Error(_) => {
            println!("Introduce un número por favor")
        }
    }

    match guess.cmp(&secret_number) {
        
        Ordering::Less => println!("Es más grande");
        Ordering::Great => println!("Es más pequeño");
        Ordering::Equal => println!("Ganaste!");

    }
}

// Path: guessing_game/src/main.rs
