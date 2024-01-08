// Programa que calcula cuantas semanas han pasado desde una fecha dada, hasta la fecha actual por
// defecto, pero que también puede acpetar una fecha dada por el usuario.


use chrono::prelude::*;

fn main() {
    let to_date_input = std::env::args().nth(1).expect("Por favor, ingrese una fecha en formato dd/mm/aaaa");
    let from_date_input: Option<String> = std::env::args().nth(2);
    
    let to_date = NaiveDate::parse_from_str(&to_date_input, "%d/%m/%Y").unwrap();
    let from_date = match from_date_input {
        Some(date) => NaiveDate::parse_from_str(&date, "%d/%m/%Y").unwrap(),
        None => Utc::now().date_naive(),
    };
    calculate_weeks(&from_date, &to_date);

}

fn calculate_weeks(from_date: &NaiveDate, to_date: &NaiveDate) -> u32 {

    let difference = from_date.signed_duration_since(*to_date);
    std::println!("Difference: {}", difference.num_weeks());
    return difference.num_weeks().abs() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_weeks() {
        // this test will fail if the current date is not 01/01/2020
        let from = NaiveDate::parse_from_str("01/01/2020", "%d/%m/%Y").unwrap();
        let to = NaiveDate::parse_from_str("22/01/2020", "%d/%m/%Y").unwrap();
        assert_eq!(calculate_weeks(&from, &to), 3);

    } 

}
