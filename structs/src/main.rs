// Struct -> Son como objetos con campos y métodos


#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
    name: String,
}

impl Point {
    fn distance_from_zero(&self) -> i32 {
        (&self.x.pow(2) + &self.y.pow(2)).pow(1/2)
    }
}

fn main() {
    let mut point = Point{
        x:1,
        y:1,
        name: String::from("Unit"),
    };
    println!("X: {}", point.x);
    println!("Y: {}", point.y);
    println!("Nombre del punto: {}", point.name);
    println!("Mi punto {point:?}");
    println!("Distancia desde el origen: {}", point.distance_from_zero());
    
    // Si algún campo del struct es dinámico cuando se clona o hace update, se vuelve inválido

    let point_2 = Point{
       x: 2,
       ..point // Esto lo haría inválido
       //y: 2,
       // name: String::from(&point.name[..]) // Pero sí podemos copiar la cadena
       //name: "hola"
    };

    //point.name = String::from("Otro nombre");
    println!("Intentando usar el punto original: {:?}", point);
    println!("Punto nuevo: {:?}", point_2);

}
