use std::io::{self, Write};

pub fn calc_simple_fee() {
    println!("type the deposit value: ");
    let mut user_input: String = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("error in read");
    let deposit: f64 = user_input.trim().parse::<f64>().expect("error in cast");

    println!("type the fee in percentage: ");
    let mut fee_input: String = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut fee_input)
        .expect("error in read the fee");
    let fee: f64 = fee_input
        .trim()
        .parse::<f64>()
        .expect("error in cast te fee input");

    let income: f64 = (fee / 100.00) * deposit;
    let total: f64 = deposit + income;

    println!("this is the total earned: {}", total);
}
