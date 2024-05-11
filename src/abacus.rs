use rusty_money::iso::Currency;
use rusty_money::{iso, Money};

/// Calculate allocated result
pub fn calculate(input: &[String]) -> Vec<Money<Currency>> {
    println!("Total to be allocated: {}", &input[0]);
    let total = Money::from_str(&input[0], iso::TWD).unwrap(); // total money to be allocated
    let mut ratios: Vec<i32> = vec![]; // allocated ratio, also is subtotal of each one

    // make ratios
    for i in 1..input.len() {
        ratios.push((&input[i]).parse::<i32>().unwrap());
    }

    total.allocate(ratios).unwrap()
}
