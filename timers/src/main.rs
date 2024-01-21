// This program is a simple timer that counts down from a specified number of seconds.
use std::thread;
use std::time::Duration;

fn main() {
    println!("------");
    println!("Bienvenido al temporizador ⏰ de Rust 🦀");

    // Wait X Seconds
    loop {
        println!("¿Cuántos segundos *quieres* esperar? - Si quieres salir presiona Ctrl+C");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("No se pudo leer la línea");

        let seconds: u64 = input
            .trim()
            .parse()
            .expect("Error al convertir el número: ");

        if seconds > 0 {
            println!("Esperando {} segundos...", seconds);
            thread::sleep(Duration::from_secs(seconds));
            println!("¡Han pasado {} segundos!", seconds);
        }
    }
}
