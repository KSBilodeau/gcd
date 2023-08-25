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

// TODO: REMOVE ALL FOLLOWING TESTS FOR BEING USELESS ONCE FEATURES ARE IMPLEMENTED

#[test]
#[should_panic]
fn euclid_unimplemented() {
    let _ = euclid_gcd(1, 1);
}

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