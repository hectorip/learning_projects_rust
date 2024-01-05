// Programa que calcula cuantas semanas han pasado desde una fecha dada, hasta la fecha actual por
// defecto, pero que también puede acpetar una fecha dada por el usuario.


use chrono::prelude::*;

fn main() {
    let input = std::env::args().nth(1).expect("Por favor, ingrese una fecha en formato dd/mm/aaaa");
    calculate_weeks(&input);
}

fn calculate_weeks(date: &str) -> u32 {
    let mut weeks = 0;
    // Get current date
    let current_date = Utc::today();
    let from_date = NaiveDate::parse_from_str(date, "%d/%m/%Y").unwrap();
    std::println!("Current date: {}", current_date);
    std::println!("From date: {}", from_date);
    // Get the difference between dates

    let difference = current_date.signed_duration_since(&from_date as &Date<Tz>);
    std::println!("Difference: {}", difference);
    // difference.num_weeks() as u32

    return 1;
}
