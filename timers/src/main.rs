// This program is a simple timer that counts down from a specified number of seconds.

fn main() {
    println!("------");
    println!("\n\n\n");
    println!("Bienvenido al temporizador");

    // Wait 10 seconds
    while true {
        println!("¿Cuántos segundos *quieres* esperar?");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("No se pudo leer la línea 💀");

        let seconds: u64 = input.trim().parse().expect("Error al convertir el número: ");
        if seconds > 0 {
            println!("Esperando {} segundos...", seconds);
            std::thread::sleep(std::time::Duration::from_secs(seconds));
            println!("¡Han pasado {} segundos!", seconds);
            // break;
        }
        println!("El número debe ser mayor a 0");
    }
}

// Shingo Nakamura - Hakodate (SineRider Remix)
// https://www.youtube.com/watch?v=ZQ7r3mZzXO0