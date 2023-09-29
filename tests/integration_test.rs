use gcd::*;

/// Tests the Consecutive Integer Method's undefined value case to ensure it errors correctly
#[test]
fn consecutive_undefined() {
    // Asserts that the function returns the appropriate error
    assert_eq!(Err("Consecutive GCD undefined for any 0 input"), consecutive_gcd(0, 0));
}

/// Tests Middle School Procedure's undefined value case to ensure it errors correctly
#[test]
fn middle_school_undefined() {
    // Asserts that the function returns the appropriate error
    assert_eq!(Err("Middle school procedure is undefined for 0 and 0"), middle_school_gcd(0, 0));
}

// Euclid's method should return b when a is 0, which this confirms
#[test]
fn euclid_zero_a() {
    assert_eq!(Ok(2), euclid_gcd(0, 2));
}

// Euclid's method should also return a when b is 0, which this confirms
#[test]
fn euclid_zero_b() {
    assert_eq!(Ok(2), euclid_gcd(2, 0));
}

// Tests the euclid_gcd against all permutations of 100 consecutive numbers
#[test]
fn euclid_one_to_hundred() {
    for a in -100..=100 {
        for b in -100..=100 {
            if a == 0 && b == 0 {
                continue;
            }

            // Brings in a trusted 3rd party dependency to check against
            // Under dev deps so it is not utilized in the actual library code
            assert_eq!(Ok(num_integer::gcd(a, b)), euclid_gcd(a, b));
        }
    }
}

// Tests the euclid_gcd against 100 random pairs of numbers ranging from 1 to
// 18_446_744_073_709_551_615
#[test]
fn euclid_random() {
    // Trusted 3rd party dependency used to generate random numbers
    use rand::Rng;

    // Create the random number generator
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        // Generate the first random number
        let a: i64 = rng.gen_range(1..i64::MAX);
        // Then the second random number
        let b: i64 = rng.gen_range(1..i64::MAX);

        // Finally compare the two algorithms to ensure they match
        assert_eq!(Ok(num_integer::gcd(a, b)), euclid_gcd(a, b));
    }
}

// From here on out, the tests follow the same pattern as euclid_gcd with minor alterations
// to account for different preconditions.

#[test]
fn consecutive_zero_a() {
    assert_eq!(Err("Consecutive GCD undefined for any 0 input"), consecutive_gcd(0, 2));
}

#[test]
fn consecutive_zero_b() {
    assert_eq!(Err("Consecutive GCD undefined for any 0 input"), consecutive_gcd(2, 0));
}

#[test]
fn consecutive_one_to_hundred() {
    for a in 1..=100 {
        for b in 1..=100 {
            assert_eq!(Ok(num_integer::gcd(a, b)), consecutive_gcd(a, b));
        }
    }
}

#[test]
fn consecutive_random() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let a: u64 = rng.gen_range(1..1000);
        let b: u64 = rng.gen_range(1..1000);

        assert_eq!(Ok(num_integer::gcd(a, b)), consecutive_gcd(a, b));
    }
}

// Determinate test that ensures that for some fixed value (100) the prime sieve works in
// comparison to some 3rd party implementation.
#[test]
fn prime_sieve_to_100() {
    // Trait that allows the sieve to be converted to an iterator
    use primes::PrimeSet;

    // Create a new 3rd party sieve for testing purposes
    let mut sieve = primes::Sieve::new();
    // Take some number of primes out that will result in primes larger than 100
    // Then filter for primes less than 100 and store in a vec
    let primes: Vec<u64> = sieve.iter().take(100).filter(|&x| x <= 100).collect();

    // Compare 3rd party prime sieve to mine
    assert_eq!(primes, prime_sieve(100));
}

// Randomly selects some upper bound n and adds variety to ensure testing is not missing potential
// edge cases. Capped at 1000 attempts with a 1..10,000 range for the sake of prime sieves taking
// forever the more primes you calculate.
#[test]
fn prime_sieve_random() {
    // Trait that allows the sieve to be converted to an iterator
    use primes::PrimeSet;
    // Trait that allows for the generation of random numbers
    use rand::Rng;

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Run this test 1000 times to ensure randomness doesn't work against the test
    for _ in 0..1000 {
        // Generate some new number n between 1 and 10,000 (lhs inclusive, rhs exclusive)
        let n: u64 = rng.gen_range(1..10000);

        // Create a new 3rd party sieve for testing purposes
        let mut sieve = primes::Sieve::new();
        // Take some number of primes out that will result in primes larger than 100
        // Then filter for primes less than 100 and store in a vec
        let primes: Vec<u64> = sieve.iter().take(n as usize).filter(|&x| x <= n).collect();

        // Compare 3rd party prime sieve to mine
        assert_eq!(primes, prime_sieve(n));
    }
}

#[test]
fn middle_school_zero_a() {
    assert_eq!(Err("Middle school procedure is undefined for 0 and 0"), middle_school_gcd(0, 2));
}

#[test]
fn middle_school_zero_b() {
    assert_eq!(Err("Middle school procedure is undefined for 0 and 0"), middle_school_gcd(2, 0));
}

#[test]
fn middle_school_one_to_hundred() {
    for a in 1..=100 {
        for b in 1..=100 {
            assert_eq!(Ok(num_integer::gcd(a, b)), middle_school_gcd(a, b));
        }
    }
}

#[test]
fn middle_school_random() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let a: u64 = rng.gen_range(1..1000) as u64;
        let b: u64 = rng.gen_range(1..1000) as u64;

        assert_eq!(Ok(num_integer::gcd(a, b)), middle_school_gcd(a, b));
    }
}