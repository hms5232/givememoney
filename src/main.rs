use std::{env, io};

use rusty_money::{Money, iso};

fn main() {
    let args: Vec<String> = env::args().collect();

    check_input(&args).expect("Bad arguments: Non-integer found");  // make sure all input is number

    println!("Total to be allocated: {}", &args[1]);
    let total = Money::from_str(&args[1], iso::TWD).unwrap();  // total money to be allocated
    let mut ratios: Vec<i32> = vec![];  // allocated ratio, also is subtotal of each one
    // make ratios
    for i in 2..args.len() {
        ratios.push((&args[i]).parse::<i32>().unwrap());
    }
    let allocated = total.allocate(ratios).unwrap();  // allocated result (sorted as ratios)
    // print result with origin price
    for i in 2..args.len() {
        println!("No.{} join allocated event with ${} and should pay ${}", i -1 , &args[i], allocated.get(i-2).unwrap().amount());
    }
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
