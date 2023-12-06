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
    // Do not know what this does
    for c in chars {
        if c == '(' || c == ')' || c == '|' || c == '*' {
            if token.len() > 0 {
                tokens.push(token);
                token = String::new();
            }
            tokens.push(c.to_string());
        } else {
            token.push(c);
        }
    }
}
