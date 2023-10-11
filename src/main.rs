use gcd::{gcd_with_algorithm, GcdAlgorithms};

fn main() {
    // Get all of the program arguments passed by the terminal
    let args: Vec<String> = std::env::args().collect();

    // Ensure that two numbers were passed to the program
    if args.len() < 3 || args.len() > 3 {
        println!("Must pass only two integers to get GCD");
    } else {
        // Match on the returned result to get either the value or the error
        // Take the first and second argument passed in by the user and parse them into
        // integers with 64 bits
        match gcd_with_algorithm(args[1].parse().unwrap(), args[2].parse().unwrap(), GcdAlgorithms::Euclid) {
            // If gcd completed without errors, print the gcd
            Ok(val) => println!("Euclid:       {}", val),
            // If gcd completed with errors, print the error
            Err(err) => println!("{}", err)
        }

        match gcd_with_algorithm(args[1].parse().unwrap(), args[2].parse().unwrap(), GcdAlgorithms::Consecutive) {
            // If gcd completed without errors, print the gcd
            Ok(val) => println!("Consecutive:  {}", val),
            // If gcd completed with errors, print the error
            Err(err) => println!("{}", err)
        }

        match gcd_with_algorithm(args[1].parse().unwrap(), args[2].parse().unwrap(), GcdAlgorithms::Middle) {
            // If gcd completed without errors, print the gcd
            Ok(val) => println!("Middle:       {}", val),
            // If gcd completed with errors, print the error
            Err(err) => println!("{}", err)
        }
    }
}
