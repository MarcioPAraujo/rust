use std::io::{self, Write};
mod average_calc;
mod fees;
fn main() {
    println!("1 - average calc");
    println!("2 - fee calculation");
    let mut option = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut option)
        .expect("error in read the option");
    match option.trim() {
        "1" => average_calc::average_calc(),
        "2" => fees::calc_simple_fee(),
        _ => println!("Invalid option"),
    }
}
