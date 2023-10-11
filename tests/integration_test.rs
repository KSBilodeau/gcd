// Import all of our gcd functions for testing
use gcd::*;

// Test the first defined 0 input case using trusted 3rd party library
#[test]
fn gcd_zero_a() {
    assert_eq!(Ok(num_integer::gcd(0, 10)), gcd(0, 10));
}

// Test the second defined 0 input case using trusted 3rd party library
#[test]
fn gcd_zero_b() {
    assert_eq!(Ok(num_integer::gcd(10, 0)), gcd(10, 0));
}

// Tests the gcd against all permutations numbers ranging from 0 to 100
#[test]
fn gcd_one_to_hundred() {
    // Loop through all permutations
    for a in 0..=100 {
        for b in 0..=100 {
            // Check for correctness in the undefined case
            if a == 0 && b == 0 {
                assert_eq!(Err("GCD is undefined for input 0 and 0."), gcd(a, b));
                continue;
            }

            // Check for correctness in the defined cases using trusted 3rd party library
            assert_eq!(Ok(num_integer::gcd(a, b) as u64), gcd(a, b));
        }
    }
}

// Tests the euclid_gcd against all permutations numbers ranging from -100 to 0
#[test]
fn gcd_neg_one_to_hundred() {
    for a in -100..=0 {
        for b in -100..=0 {
            // Check for correctness in the undefined case
            if a == 0 && b == 0 {
                assert_eq!(Err("GCD is undefined for input 0 and 0."), gcd(a, b));
                continue;
            }

            // Check for correctness in the defined cases using trusted 3rd party library
            assert_eq!(Ok(num_integer::gcd(a, b) as u64), gcd(a, b));
        }
    }
}


// Tests the gcd against 100 random sets of numbers ranging from -100000 to 100000
#[test]
fn gcd_random() {
    // Trusted 3rd party dependency used to generate random numbers
    use rand::Rng;

    // Create the random number generator
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        // Generate the first random number
        let a: i64 = rng.gen_range(-100000..100000);
        // Then the second random number
        let b: i64 = rng.gen_range(-100000..100000);

        // Finally compare the two algorithms to ensure they match
        assert_eq!(Ok(num_integer::gcd(a, b) as u64), gcd(a, b));
    }
}