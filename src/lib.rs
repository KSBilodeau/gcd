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
//!         if let Ok(result) = euclid_gcd(a, b) {
//!             println!("GCD of {a} and {b} is {result}.");
//!         }
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
/// if let Ok(result) = euclid_gcd(15, 25) {
///     println!("The GCD of 15 and 25 is {result}!")
/// }
/// # }
pub fn euclid_gcd(a: u64, b: u64) -> Result<u64, &'static str> {
    // Check for the undefined case, followed by the base cases, and ending with the recursive case
    if a == 0 && b == 0 {
        // Error on the undefined case
        Err("GCD undefined for 0 and 0")
    } else if a == 0 {
        // Return the non-zero answer
        Ok(b)
    } else if b == 0 {
        // Return the non-zero answer
        Ok(a)
    } else {
        // Apply Euclid's Method as described in lecture
        euclid_gcd(b, a % b)
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

/// Uses the algorithm known as the Sieve of Eratosthenes.
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
/// ```should_panic
/// # use gcd::*;
/// #
/// # fn main() {
/// // Extract the result from the function as we know it will be Ok as the GCD is not undefined
/// if let Ok(result) = middle_school_gcd(15, 25) {
///     println!("The GCD of 15 and 25 is {result}!")
/// }
/// # }
pub fn middle_school_gcd(a: u64, b: u64) -> Result<u64, &'static str> {
    if a == 0 && b == 0 {
        Err("GCD undefined for 0 and 0")
    } else {
        todo!()
    }
}