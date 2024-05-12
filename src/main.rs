use std::{env, io};

mod mission;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Current not support interaction mode
    if args.len() < 2 {
        println!("givememoney v{}", env!("CARGO_PKG_VERSION"));
        println!("To allocate money, input `gmm [total] [each participant separate by space]`");
        return;
    }
    check_input(&args).expect("Bad arguments: Non-integer found"); // make sure all input is number

    mission::Round::new(&args[1..args.len()])
        .allocate()
        .display();
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
