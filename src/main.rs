use gcd::euclid_gcd;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Must pass two integers to get GCD");
    } else {
        if args.len() == 3 {
            println!("{}", euclid_gcd(args[1].parse().unwrap(), args[2].parse().unwrap(), &mut 1, &mut 2));
        } else {
            println!("Not enough arguments")
        }
    }
}
