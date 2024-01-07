// Programa que calcula cuantas semanas han pasado desde una fecha dada, hasta la fecha actual por
// defecto, pero que también puede acpetar una fecha dada por el usuario.


use chrono::prelude::*;

fn main() {
    let input = std::env::args().nth(1).expect("Por favor, ingrese una fecha en formato dd/mm/aaaa");
    calculate_weeks(&input);
}

fn calculate_weeks(from_date: NaiveDate, to_date: NaiveDate) -> u32 {
    // Get current date
    let current_date = Utc::now().date_naive();
    let from_date = NaiveDate::parse_from_str(date, "%d/%m/%Y").unwrap();
    std::println!("Current date: {}", current_date);
    std::println!("From date: {}", from_date);
    // Get the difference between dates

    let difference = current_date.signed_duration_since(from_date);
    std::println!("Difference: {}", difference.num_weeks());
    // difference.num_weeks() as u32

    return difference.num_weeks() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_weeks() {
        // this test will fail if the current date is not 01/01/2020
        assert_eq!(calculate_weeks("01/01/2020"), 209);
    }

}
