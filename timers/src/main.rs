// This program is a simple timer that counts down from a specified number of seconds.
use std::thread;
use std::time::Duration;


fn main() {
    println!("------");
    println!("\n\n\n");
    println!("Bienvenido al temporizador");

    // Wait 10 seconds
    loop {
        println!("¿Cuántos segundos *quieres* esperar?");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("No se pudo leer la línea");

        let seconds: u64 = input.trim().parse().expect("Error al convertir el número: ");
        if seconds > 0 {
            println!("Esperando {} segundos...", seconds);
            thread::sleep(Duration::from_secs(seconds));
            println!("¡Han pasado {} segundos!", seconds);
            // break;
        }
    }
}
