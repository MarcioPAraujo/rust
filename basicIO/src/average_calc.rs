use std::io::{self, Write};

pub fn average_calc() {
    println!("let's calc a simple average");
    println!("type the first number: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).expect("invalid value");
    let number1: f64 = input1.trim().parse::<f64>().expect("invalid value");

    println!("type the second number: ");
    let mut input2: String = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).expect("error in read");
    let number2: f64 = input2.trim().parse::<f64>().expect("invalid value");

    let total_sum: f64 = number1 + number2;
    let average: f64 = total_sum / 2.0;

    println!("the average is: {}", average);
}
