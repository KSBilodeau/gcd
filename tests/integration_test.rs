use gcd::*;

/// Tests Euclid Method's undefined value case to ensure it errors correctly
#[test]
fn euclid_undefined() {
    // Asserts that the function returns the appropriate error
    assert_eq!(Err("GCD undefined for 0 and 0"), euclid_gcd(0, 0));
}

/// Tests the Consecutive Integer Method's undefined value case to ensure it errors correctly
#[test]
fn consecutive_undefined() {
    // Asserts that the function returns the appropriate error
    assert_eq!(Err("GCD undefined for 0 and 0"), consecutive_gcd(0, 0));
}

/// Tests Middle School Procedure's undefined value case to ensure it errors correctly
#[test]
fn middle_school_undefined() {
    // Asserts that the function returns the appropriate error
    assert_eq!(Err("GCD undefined for 0 and 0"), middle_school_gcd(0, 0));
}

#[test]
fn euclid_zero_a() {
    assert_eq!(Ok(2), euclid_gcd(0, 2));
}

#[test]
fn euclid_zero_b() {
    assert_eq!(Ok(2), euclid_gcd(2, 0));
}

#[test]
fn euclid_one_to_hundred() {
    for a in 1..=100 {
        for b in 1..=100 {
            assert_eq!(Ok(num_integer::gcd(a, b)), euclid_gcd(a, b));
        }
    }
}

#[test]
fn euclid_random() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let a: u64 = rng.gen_range(1..u64::MAX);
        let b: u64 = rng.gen_range(1..u64::MAX);

        assert_eq!(Ok(num_integer::gcd(a, b)), euclid_gcd(a, b));
    }
}

// TODO: REMOVE ALL FOLLOWING TESTS FOR BEING USELESS ONCE FEATURES ARE IMPLEMENTED

#[test]
#[should_panic]
fn consecutive_unimplemented() {
    let _ = consecutive_gcd(1, 1);
}

#[test]
#[should_panic]
fn middle_school_unimplemented() {
    let _ = middle_school_gcd(1, 1);
}