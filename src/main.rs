use std::{env, io};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    check_input(&args).expect("Bad arguments.");  // make sure all input is number

    let total = &args[1];  // total money to be allocated
    let mut member: HashMap<usize, i32> = HashMap::new();
    println!("Total to be allocated: {}", total);
    for i in 2..args.len() {
        println!("No.{} to join allocated event with {}", i -1 , &args[i]);
        member.insert(i - 1, (&args[i]).parse::<i32>().unwrap());
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
