fn main() {
    println!("Probando tipos y variables, junto con construcciones básicas. 🦾");

    // Variable inmutable de tipo entero positivo de 32 bits
    let edad: u32 = 932;
    // Variable mutable de tipo entero con signo de 32 bits
    // Existen enteros con signo (i) y sin signo (u) de 8, 16, 32 y 64 bits
    
    let mut temperatura: i32 = -34;
    temperatura += 1;
    println!("Edad: {edad} <-> Temperatura: {temperatura}");

    // Carácteres - puede ser cualquier carácter Unicode, ocupan 4 bytes
    let una_letra: char = '🤌'; // Se declaran con comillas sencillas
    println!("Mi carácter {una_letra}");

    // Booleanos
    let es_mentira = false;
    let es_verdad = true;
    println!("¿Es mentira? {es_mentira} - ¿Es verdad? {es_verdad}");
}
