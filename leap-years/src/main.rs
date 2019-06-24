use std::io;
use leap_years::Year;

fn main() {
    println!("Enter year.");
    let mut year_query = String::new();
    io::stdin()
        .read_line(&mut year_query)
        .expect("Failed to read line");
    let year_query: u32 = year_query.trim().parse().expect("Invalid year format");
    let year = Year::new(year_query);
    match year.is_leap() {
        Ok(true) => print!("Year {} is leap\n", year_query),
        Ok(false) => print!("Year {} is not leap\n", year_query),
        Err(err) => print!("{}\n", err),
    }
}
