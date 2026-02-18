use std::env;
use std::io::Write;

mod mission;
mod money;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let run_without_parameters = args.len() < 2;

    // If no parameters or not enough parameters, show usage and wait for user input
    while args.len() < 3 {
        println!(
            "\
            givememoney v{}\n\
            To allocate money, input `gmm [total] [each participant separate by space]`\n\
            \n\
            Check {} for more information and update.\
            ",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_REPOSITORY"),
        );

        // show "gmm " and wait for user input money args
        println!();
        print!("> gmm ");
        std::io::stdout().flush().unwrap();
        // get user input and write to args
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        args = input.split_whitespace().map(|s| s.to_string()).collect();
        args.insert(0, "gmm ".to_string()); // make args like the usage from Unix-like target
    }
    // make sure all inputs are number or valid format: name=number
    if check_input(&args[1..]).is_err() {
        if run_without_parameters {
            press_enter_to_exit();
        }
        return;
    }

    mission::Round::new(&args[1..args.len()])
        .allocate()
        .display();

    if run_without_parameters {
        press_enter_to_exit()
    }
}

/// Check if all arguments are number or valid format.
///
/// # Arguments
///
/// * `args` - Input needs check
///
/// # Errors
///
/// If input is not number or valid format, return `io::ErrorKind::Other`
///
/// # Examples
///
/// ```rust
/// assert!(check_input(["100".to_string(), "40".to_string(), "70".to_string()]).is_ok());
///
/// assert!(check_input(["100".to_string(), "Alice=40".to_string(), "70".to_string()]).is_ok());
///
/// assert!(check_input(["100".to_string(), "40".to_string(), "Bob".to_string()]).is_err());
///
/// assert!(check_input(["Hi".to_string(), "100".to_string(), "40".to_string(), "Bob".to_string()]).is_err());
/// ```
fn check_input(args: &[String]) -> Result<(), &str> {
    // check total (first argument)
    if !is_natural_number(args.get(0).unwrap()) {
        eprintln!("The first argument must be total amount of money, should not with name.");
        return Err("Bad argument: the first argument should be total.");
    }
    // check player(s)
    for (n, i) in args.iter().skip(1).enumerate() {
        let mut money_from_input = i.as_str();
        // specify the participant name
        if i.contains('=') {
            money_from_input = i.split('=').collect::<Vec<_>>()[1];
        }
        if !is_natural_number(money_from_input) {
            eprintln!(
                "Unable to parse number from argument (position: {}): {}",
                n + 1,
                i
            );
            return Err("Bad argument: non-integer found.");
        }
    }
    Ok(())
}

/// Check given value is natural number or not.
fn is_natural_number(value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(_number) => true,
        Err(_e) => false,
    }
}

/// Show "Press enter to exit" and wait for user input
fn press_enter_to_exit() {
    #[cfg(not(target_os = "windows"))]
    {
        return;
    }
    #[cfg(target_os = "windows")]
    {
        println!("Press enter to exit.");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}

#[cfg(test)]
mod test_fn_check_input {
    use super::check_input;

    #[test]
    fn input_number() {
        let mut input: Vec<String> = vec![String::from("100")];
        input.push(String::from("40"));
        input.push(String::from("70"));

        assert!(check_input(&input).is_ok());
    }

    #[test]
    fn input_with_name() {
        let mut input: Vec<String> = vec![String::from("100")];
        input.push(String::from("Alex=40"));
        input.push(String::from("70"));

        assert!(check_input(&input).is_ok());
    }

    #[test]
    fn input_non_number() {
        let mut input: Vec<String> = vec![String::from("100")];
        input.push(String::from("Vicky"));
        input.push(String::from("70"));

        assert!(check_input(&input).is_err());
    }
}
