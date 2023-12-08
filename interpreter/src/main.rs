// This will be a simple interpreter that will take in a string and print it out.
fn main() {
    // read program from file in the first argument
    let program = std::env::args().nth(1).expect("Please provide a program");

    println!("Running the interpreter");
    println!("Program: {:?}", program);
}
