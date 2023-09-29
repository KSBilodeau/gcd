//! Simple, fallible, and fun: Greatest common divisors made easy.
//!
//! This is a demonstration of three methods of calculating the greatest common divisor.  The
//! algorithms used are as follows:
//! - Euclid's Method
//! - Consecutive Integer Checking Method
//! - Middle School Procedure
//!
//! # Example
//! The following example illustrates how to calculate the greatest common divisors (GCDs) for
//! permutations of numbers ranging from 1 to 100:
//! ```
//! # use gcd::*;
//! #
//! # fn main() {
//! for a in 1..100 {
//!     for b in 1..100 {
//!         println!("GCD of {a} and {b} is {}.", euclid_gcd(a, b));
//!     }
//! }
//! # }
//! ```

/// Uses Euclid's Method of finding the GCD of two numbers.
///
/// Euclid's Method is an incredibly simple algorithm for determining the Greatest Common Divisor
/// (GCD) of two numbers.  The method can be applied iteratively or recursively; however, the
/// recursive method is the simplest.
///
/// As with all of the algorithms provided in this library, this operates under the
/// assumption that all given values are real numbers greater than 0.
///
/// # Errors
/// Will return `Err` if the GCD for two given numbers is undefined.
///
/// # Example
/// ```
/// # use gcd::*;
/// #
/// # fn main() {
/// // Extract the result from the function as we know it will be Ok as the GCD is not undefined
/// println!("The GCD of 15 and 25 is {}!", euclid_gcd(15, 25).unwrap())
/// # }
/// ```
pub fn euclid_gcd(a: i64, b: i64) -> Result<i64, &'static str> {
    if a == 0 && b == 0 {
        Err("GCD undefined for input of 0 and 0.")
    } else {
        let (mut s1, mut t1, mut r1) = (0, 1, b);
        let (mut s2, mut t2, mut r2) = (1, 0, a);

        while r1 != 0 {
            let quotient = r2 / r1;

            (r2, r1) = (r1, r2 - quotient * r1);
            (s2, s1) = (s1, s2 - quotient * s1);
            (t2, t1) = (t1, t2 - quotient * t1);
        }

        Ok(r2.abs())
    }
}

/// Uses the Consecutive Integer Method of finding the GCD of two numbers.
///
/// The Consecutive Integer Method functions about how you'd expect it to. It first
/// identifies the larger number and then decreases it by one until it reaches a number that divides
/// evenly into both dividends.
///
/// As with all of the algorithms provided in this library, this operates under the
/// assumption that all given values are real numbers greater than 0.
///
/// # Errors
/// Will return `Err` if the GCD for two given numbers is undefined.
///
/// # Example
/// ```
/// # use gcd::*;
/// #
/// # fn main() {
/// // Extract the result from the function as we know it will be Ok as the GCD is not undefined
/// if let Ok(result) = consecutive_gcd(15, 25) {
///     println!("The GCD of 15 and 25 is {result}!")
/// }
/// # }
/// ```
pub fn consecutive_gcd(a: u64, b: u64) -> Result<u64, &'static str> {
    if a == 0 || b == 0 {
        Err("Consecutive GCD undefined for any 0 input")
    } else {
        let mut t = std::cmp::min(a, b);

        loop {
            if a % t == 0 {
                if b % t == 0 {
                    return Ok(t);
                }
            }

            t -= 1;
        }
    }
}

/// Uses the algorithm known as the Sieve of Eratosthenes to determine primes up to n.
///
/// The Sieve of Eratosthenes is an iterative algorithm that
/// repeatedly goes over the length of 2..n until it has eliminated all
/// non-prime numbers.  It does this by going down an array of 0,0,2..n integers
/// and setting multiples of each number that is not zero to zero, so they are skipped
/// over.
///
/// # Example
/// ```
/// # use gcd::*;
/// #
/// # fn main() {
/// println!("The set of primes for n <= 25 includes: {:?}", prime_sieve(25));
/// // Output: "The set of primes for n <= 25 includes: [2, 3, 5, 7, 11, 13, 17, 19, 23]"
/// # }
/// ```
pub fn prime_sieve(n: u64) -> Vec<u64> {
    let mut primes = [0, 0].into_iter().chain(2..=n).collect::<Vec<_>>();

    for prime in 2..=(f64::sqrt(n as f64).floor() as u64) {
        if primes[prime as usize] != 0 {
            let mut step = prime * prime;

            while step <= n {
                primes[step as usize] = 0;
                step += prime;
            }
        }
    }

    primes.iter()
        .filter_map(|&x| if x != 0 { Some(x) } else { None })
        .collect()
}

/// Gives the occurrences of a given prime in a factorization of some number n.
///
/// This doesn't use any specific algorithm.  It simply loops adding to an
/// internal occurrence counter until the prime cannot divide into n anymore.
///
/// # Panic
/// Panics when n is not able to be evenly divisible by prime.
///
/// # Example
/// ```
/// # use gcd::*;
/// #
/// # fn main() {
/// println!("{} twos in 60.", occurrences(60, 2));
/// // Output: "2 twos in 60"
/// # }
/// ```
pub fn occurrences(n: u64, prime: u64) -> u64 {
    if n % prime != 0 {
        panic!("Cannot find number of occurrences for non-factor prime.");
    }

    let mut occurrences = 1;
    let mut leftovers = n / prime;

    while leftovers % prime == 0 {
        occurrences += 1;
        leftovers = leftovers / prime;
    }

    occurrences
}

/// Gives the prime factorization of any given number n.
///
/// The algorithm filters the output from prime_sieve for n and then uses occurrences to bundle
/// the number of occurrences of any given factor into a tuple.
pub fn prime_factors(n: u64) -> Vec<(u64, u64)> {
    // Sieve the number to get all of the primes up to n
    prime_sieve(n)
        // Start an iterator chain
        .iter()
        // Filter for factors and then combine them with their occurrences
        .filter_map(|&x| if n % x == 0 { Some((x, occurrences(n, x))) } else { None })
        // Collect it into a vec and return
        .collect()
}

/// Uses Middle School Procedure to find the GCD of two numbers.
///
/// Middle School procedure is the most simplistic out of all the algorithms for GCD. It does
/// exactly what is taught in middle school in that you find all the factors of both numbers. Then
/// you compare and find the largest factor that exists in both sets of factors.
///
/// As with all of the algorithms provided in this library, this operates under the
/// assumption that all given values are real numbers greater than 0.
///
/// # Errors
/// Will return `Err` if the GCD for two given numbers is undefined.
///
/// # Example
/// ```
/// # use gcd::*;
/// #
/// # fn main() {
/// // Extract the result from the function as we know it will be Ok as the GCD is not undefined
/// if let Ok(result) = middle_school_gcd(15, 25) {
///     println!("The GCD of 15 and 25 is {result}!")
/// }
/// # }
/// ```
pub fn middle_school_gcd(a: u64, b: u64) -> Result<u64, &'static str> {
    // If the GCD is for 0 and 0, return an error
    if a == 0 || b == 0 {
        Err("Middle school procedure is undefined for 0 and 0")
    } else {
        // Get the prime factors and their occurrences for both term a and b
        let factors_a = prime_factors(a);
        let factors_b = prime_factors(b);

        // Use my O(min(M, N)) vector intersection algorithm from the first homework assignment

        // Two indexes to keep track of where we are in both vectors
        let mut index_a = 0;
        let mut index_b = 0;

        // Vector to store results
        let mut intersection = vec![];

        // Keep looping until we hit the end of one of the vectors
        while index_a < factors_a.len() && index_b < factors_b.len() {
            // If the primes are equal, push the factor onto the intersection list (ensuring the
            // min of the two exponents is pushed with the factor
            // Otherwise, move the indexes in accordance to whether A or B factor is greater
            if factors_a[index_a].0 == factors_b[index_b].0 {
                intersection.push(
                    (factors_a[index_a].0, std::cmp::min(factors_a[index_a].1, factors_b[index_b].1))
                );

                index_a += 1;
                index_b += 1;
            } else if factors_a[index_a].0 > factors_b[index_b].0 {
                index_b += 1;
            } else {
                index_a += 1;
            }
        }

        // Use the fold function to collapse the vector down to a single value,
        // which is our result
        Ok(
            intersection
                .iter()
                .fold(1, |acc, &(prime, exponent)| {
                    acc * prime.pow(exponent as u32)
                })
        )
    }
}