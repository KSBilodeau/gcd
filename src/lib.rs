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
    if a == 0 && b == 0 {
        Err("GCD undefined for 0 and 0")
    } else {
        Ok(0)
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
    if a == 0 && b == 0 {
        Err("GCD undefined for 0 and 0")
    } else {
        Ok(0)
    }
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
pub fn middle_school_gcd(a: u64, b: u64) -> Result<u64, &'static str> {
    if a == 0 && b == 0 {
        Err("GCD undefined for 0 and 0")
    } else {
        Ok(0)
    }
}