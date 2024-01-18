
fn main() {
    // Explorando los tres diferentes tipos de ciclos que tiene Rust
    //
    // Ciclo infinito
    //
    let mut i = 0;
    loop {
        i += 1;
        println!("Loop: Voy a correr mientras el programa no se detenga o se llame a 'break'");
        println!("Ciclo: {i}");
        if i == 10 {
            break;
        }
    }

    let sep = "*".repeat(50);
    println!("{}", sep);
    while i < 20 {
        println!("While: voy a correr mientras una condición se cumpla");
        println!("Ciclo: {i}");
        i += 1;
    }
    println!("{}", sep);
    for x in 1..=10{
        println!("For: voy a recorrer una colección hasta terminarla.");
        println!("Elemento actual: {x}");
    
    }
}
