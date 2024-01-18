
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32
}

impl Point {

    fn distance_from_zero(&self) -> i32 {
        (&self.x.pow(2) + &self.y.pow(2)).pow(1/2)
    }
}


fn main() {
    let point = Point{
        x:1,
        y:1,
    };
    println!("X: {}", point.x);
    println!("Y: {}", point.y);
    println!("Mi punto {point:?}");
    println!("Distancia desde el origen: {}", point.distance_from_zero());
}
