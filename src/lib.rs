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
//! for a in -100..100 {
//!     for b in -100..100 {
//!         if a == 0 && b == 0 {
//!             println!("GCD of 0 and 0 is undefined.");
//!             continue;
//!         }
//!
//!         println!("GCD of {a} and {b} is {}.", gcd(a, b).unwrap());
//!     }
//! }
//! # }
//! ```

/// Represents all possible algorithms available in this library. Used for algorithm selection
/// in gcd_with_algorithm.
pub enum GcdAlgorithms {
    Euclid,
    Middle,
    Consecutive,
}

/// Uses provided algorithm to calculate the gcd of two numbers.
///
/// Given the algorithm, this function calculates the result using the while
/// also upholding the given invariants of all algorithms in this library.
///
/// # Errors
///
/// When a and b are zero, the function returns a GCD undefined error.
pub fn gcd_with_algorithm(a: i64, b: i64, algo: GcdAlgorithms) -> Result<u64, &'static str> {
    // Set the operands to the absolute versions of themselves, as the GCD for two integers a and b
    // where a or b ∈ ℤ- will always equal the GCD of |a| and |b| where a and b are two integers
    let a = a.abs() as u64;
    let b = b.abs() as u64;

    // Check for the undefined case, and return according error if true
    if a == 0 && b == 0 {
        Err("GCD is undefined for input 0 and 0.")
    }
    // Check for zero input cases, and quick case it into returning the max of a and b
    else if a == 0 || b == 0 {
        Ok(std::cmp::max(a, b))
    }
    // Otherwise, apply the supplied algorithm
    else {
        match algo {
            GcdAlgorithms::Euclid => Ok(euclid_gcd(a as i64, b as i64)),
            GcdAlgorithms::Consecutive => Ok(middle_school_gcd(a, b)),
            GcdAlgorithms::Middle => Ok(middle_school_gcd(a, b)),
        }
    }
}

/// Uses all three algorithms and returns the collective result.
///
/// This function abstracts over all algorithms in this library and returns the result of the
/// combined value of all algorithms.  It does so by calling `gcd_with_algorithm` for all algorithms
/// and then comparing the results.
///
/// # Errors
///
/// Returns `Err(...)` when the result of the comparison across all algorithms
/// differs, as well as when `gcd_with_algorithm` errors.
pub fn gcd(a: i64, b: i64) -> Result<u64, &'static str> {
    // Run the euclid algorithm
    let euclid = gcd_with_algorithm(a, b, GcdAlgorithms::Euclid);

    // Run the consecutive algorithm
    let consecutive = gcd_with_algorithm(a, b, GcdAlgorithms::Consecutive);

    // Run the middle school algorithm
    let middle = gcd_with_algorithm(a, b, GcdAlgorithms::Middle);

    // Compare the algorithm's results and return accordingly
    if euclid != consecutive || euclid != middle {
        Err("GCD does not match across all algorithms.")
    } else {
        euclid
    }
}

/// Uses Euclid's Method of finding the GCD of two numbers.
///
/// Euclid's Method is an incredibly simple algorithm for determining the Greatest Common Divisor
/// (GCD) of two numbers.  The method can be applied iteratively or recursively; however, the
/// recursive method is the simplest.
///
/// # Invariants
///
/// This algorithm assumes that neither argument is 0; therefore, using such inputs is not
/// guaranteed to be correct.
pub fn euclid_gcd(a: i64, b: i64) -> u64 {
    let (mut s1, mut t1, mut r1) = (0, 1, b);
    let (mut s2, mut t2, mut r2) = (1, 0, a);

    while r1 != 0 {
        let quotient = r2 / r1;

        (r2, r1) = (r1, r2 - quotient * r1);
        (s2, s1) = (s1, s2 - quotient * s1);
        (t2, t1) = (t1, t2 - quotient * t1);
    }

    r2 as u64
}

/// Uses the Consecutive Integer Method of finding the GCD of two numbers.
///
/// The Consecutive Integer Method functions about how you'd expect it to. It first
/// identifies the larger number and then decreases it by one until it reaches a number that divides
/// evenly into both dividends.
///
/// # Panics
///
/// If either input is 0, the algorithm will panic due to division by zero.
pub fn consecutive_gcd(a: u64, b: u64) -> u64 {
    // Get the minimum of the two numbers, akin to how the euclid gcd algorithm swaps by means of
    // modulo
    let mut t = std::cmp::min(a, b);

    // Loop continuously until result is found
    loop {
        // Check if t divides into a
        if a % t == 0 {
            // Check if t divides into b
            if b % t == 0 {
                // If t divides into both, then return it as the gcd
                return t;
            }
        }

        // If t divides into neither a or b, decrement
        t -= 1;
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
/// # Panics
///
/// If the input is large enough, it may cause the internally used vectors to panic due to
/// capacity overflow.
pub fn prime_sieve(n: u64) -> Vec<u64> {
    // Create a list of numbers from 2 to n, padding the front with 0s to align the indexes to their
    // numeric values as well as indicating that 0 and 1 are not prime
    let mut primes = [0, 0].into_iter().chain(2..=n).collect::<Vec<_>>();

    // Loop through each index to mark its multiples as not prime (aka 0), skipping over
    // already disqualified numbers.  Only to sqrt of n as it allows us to guarantee that
    // skipping primes by means of prime * prime will be valid for all numbers checked.
    for prime in 2..=(f64::sqrt(n as f64).floor() as u64) {
        // If the possible prime has not already been disqualified, disqualify its multiples
        if primes[prime as usize] != 0 {
            // Instead of starting from 0 every time, start from the square of the prime as to
            // skip multiples that are guaranteed to have already been disqualified previously
            let mut step = prime * prime;

            // While step is within the range of the array, disqualify each multiple of the prime
            // after prime^2
            while step <= n {
                // Disqualify the multiple
                primes[step as usize] = 0;
                // Set step to the next multiple
                step += prime;
            }
        }
    }

    // Return a vector of all of the primes from 0 to n by turning primes into an iterator and
    // filtering it
    primes.iter()
        // Check if each value in primes is 0 or not, and returns all values that are not 0 (it also
        // copies all values to make them not 0)
        .filter_map(|&x| if x != 0 { Some(x) } else { None })
        // Turn the iterator back into a vector with a type that matches the return value
        .collect()
}

/// Gives the occurrences of a given prime in a factorization of some number n.
///
/// This doesn't use any specific algorithm.  It simply loops adding to an
/// internal occurrence counter until the prime cannot divide into n anymore.
///
/// # Panic
///
/// Panics when n is not able to be evenly divisible by prime.
pub fn occurrences(n: u64, prime: u64) -> u64 {
    // Check if the supplied prime is actually a factor of n
    if n % prime != 0 {
        // Crash the program if prime is not a factor of n
        panic!("Cannot find number of occurrences for non-factor prime.");
    }

    // Since prime is a factor of n, it occurs at least once
    let mut occurrences = 1;
    // Contains the potentially still divisible portion of n
    let mut leftovers = n / prime;

    // So long as n is still divisible by the prime
    while leftovers % prime == 0 {
        // Increase the number of times the factor occurs in n
        occurrences += 1;
        // Remove prime from n and reassign it to the leftovers to progress the loop
        leftovers = leftovers / prime;
    }

    // Return how many times a prime factor divides into a number n
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
/// # Panics
///
/// If the input is large enough, it may cause the internally used vectors to panic due to
/// capacity overflow.
///
/// # Invariants
///
/// This algorithm assumes that neither argument is 0; therefore, using such inputs is not
/// guaranteed to be correct.
pub fn middle_school_gcd(a: u64, b: u64) -> u64 {
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
        // min of the two exponents is pushed with the factor)
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
    intersection
        .iter()
        .fold(1, |acc, &(prime, exponent)| {
            acc * prime.pow(exponent as u32)
        })
}