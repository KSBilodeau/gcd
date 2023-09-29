use gcd::euclid_gcd;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Must pass two integers to get GCD");
    } else {
        if args.len() == 3 {
            // Match on the returned result to get either the value or the error
            // Absolute value as the functions assume positive integers (and result is same)
            match euclid_gcd(args[1].parse().unwrap(), args[2].parse().unwrap()) {
                Ok(val) => println!("{}", val),
                Err(err) => println!("{}", err)
            }
        } else {
            println!("Not enough arguments")
        }
    }
}
