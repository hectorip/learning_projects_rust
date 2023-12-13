fn main() {
    println!("Evaluating regular expressions");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // Create a vector of chars from the input string
    let mut chars: Vec<char> = input.trim().chars().collect();
    println!("chars: {:?}", chars);

    // Create a vector of tokens from the vector of chars
    let mut tokens: Vec<String> = Vec::new();
    let mut token: String = String::new();
}
