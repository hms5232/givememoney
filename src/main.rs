use std::{env, io};

use rusty_money::{iso, Money};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Current not support interaction mode
    if args.len() < 2 {
        println!("Too few arguments.");
        unimplemented!("Current not support interaction mode")
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
    let allocated = total.allocate(ratios).unwrap(); // allocated result (sorted as ratios)

    // print result with origin price
    for i in 1..input.len() {
        println!(
            "No.{} join allocated event with ${} and should pay ${}",
            i,
            &input[i],
            allocated.get(i - 1).unwrap().amount()
        );
    }
}
