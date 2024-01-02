use std::collections::HashMap;

// CONVIERTE UNA PALABRA EN UN NUMERO DE TELEFONO

fn main() {
    // reading the input from the first argument
    let input = std::env::args().nth(1).expect("please provide a word to convert");
    let clean_input = input.to_lowercase();
    let mut output = String::new();
    

    let dialpad = HashMap::from([
        ("a", "2"), ("b", "2"), ("c", "2"),
        ("d", "3"), ("e", "3"), ("f", "3"),
        ("g", "4"), ("h", "4"), ("i", "4"),
        ("j", "5"), ("k", "5"), ("l", "5"),
        ("m", "6"), ("n", "6"), ("o", "6"),
        ("p", "7"), ("q", "7"), ("r", "7"), ("s", "7"),
        ("t", "8"), ("u", "8"), ("v", "8"),
        ("w", "9"), ("x", "9"), ("y", "9"), ("z", "9"),
    ]);

    for c in clean_input.chars() {
        let digit = dialpad.get(&c.to_string() as &str).expect("input contains invalid characters");
        output.push_str(&digit.to_string());
    }
    println!("{} => {}", input, output);
}
