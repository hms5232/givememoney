use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use std::{env, io};

use rusty_money::{iso, Money};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Current not support interaction mode
    if args.len() < 2 {
        println!("givememoney v{}", env!("CARGO_PKG_VERSION"));
        println!("To allocate money, input `gmm [total] [each participant separate by space]`");
        return;
    }
    check_input(&args).expect("Bad arguments: Non-integer found"); // make sure all input is number

    calculate(&args[1..args.len()]);
}

/// Check if all arguments is number.
fn check_input(args: &Vec<String>) -> Result<(), io::Error> {
    for i in 1..args.len() {
        match args[i].parse::<i32>() {
            Ok(_number) => (),
            Err(e) => {
                eprintln!("Unable to parse number from argument: {}", args[i]);
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }
        }
    }
    Ok(())
}

/// Calculate and print result
fn calculate(input: &[String]) {
    println!("Total to be allocated: {}", &input[0]);
    let total = Money::from_str(&input[0], iso::TWD).unwrap(); // total money to be allocated
    let mut ratios: Vec<i32> = vec![]; // allocated ratio, also is subtotal of each one

    // make ratios
    for i in 1..input.len() {
        ratios.push((&input[i]).parse::<i32>().unwrap());
    }

    let allocated = total.allocate(ratios).unwrap(); // allocated result

    // print result with origin price
    let mut table = vec![];
    for i in 1..input.len() {
        // i => origin order from input
        // &input[i] => origin input value
        table.push(vec![
            i.cell(),
            (&input[i]).cell().justify(Justify::Right),
            allocated
                .get(i - 1)
                .unwrap()
                .amount()
                .cell()
                .justify(Justify::Right),
        ]);
    }
    // print result table
    println!(
        "{}",
        table
            .table()
            .title(vec![
                "No.".cell().bold(true),
                "Original".cell().bold(true),
                "Allocated".cell().bold(true)
            ])
            .display()
            .unwrap()
    );
}
