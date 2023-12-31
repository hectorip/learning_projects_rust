// Simulando un CPU
// 
// Instrucciones soportadas, como tuplas:
// ('ADD', 'Ra', 'Rb', 'Rc') -> Rc = Ra + Rb
// ('SUB', 'Ra', 'Rb', 'Rc') -> Rc = Ra - Rb
// ('MUL', 'Ra', 'Rb', 'Rc') -> Rc = Ra * Rb
// ('DIV', 'Ra', 'Rb', 'Rc') -> Rc = Ra / Rb
// ('CONST', 'Ra', value) -> Ra = value
// ('LOAD', 'Ra', 'Rb') -> Rb = RAM[Ra]
// ('STORE', 'Ra', 'Rb') -> RAM[Ra] = Rb
//
// Un programa es una lista de instrucciones

fn main() {
    println!("Corriendo el programa de simulación de CPU");
}

struct Instruction {

    // Instrucción
    // Operando 1
    // Operando 2
    // Operando 3
    command: String,
    operand1: String,
    operand2: String,
    operand3: String,
}


struct CPU {
    // Registros
    // Program Counter
    // RAM
}

fn cpu(programa: Vec<Instruction>) {
    // Inicializa el CPU y la RAM
    // Cómo definir la RAM?
    registros = {"Ra": 0; "Rb": 0, "Rc": 0, "Rd": 0, "Re": 0, "Rf": 0, "Rg": 0, "Rh": 0}
    // Imprime el estado de la CPU
}
