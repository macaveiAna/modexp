//! Command-line modular exponentation tool
//!
//! Ana Macavei 2023

use std::env::args;

fn main() {
    // read in user input
    let user_input: Vec<String> = args().collect();
    // assign the values to a variable.
    // start at index 1 since the name of the program is at index 0
    let x = &user_input[1];
    let y = &user_input[2];
    let m = &user_input[3];

    //convert a string to another type using the parse method
    //found in chapter 2 of the reading
    //print a usage message on stderr and stop with exit status 1
    //Chapter 6.2 for match and 9.2 recoverable errors with result
    let x: u64 = x.parse().expect("Please type a number!");
    let y: u64 = y.parse().expect("Please type a number!");
    let m: u64 = m.parse().expect("Please type a number!");

    let result = modexp(x, y, m);

    println!("The result of ({} ^ {}) % {} = {}", x, y, m, result);
}

// A function to calculate modular exponentiation: (x^y) % m
// return type: unsigned integer
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    // convert u64 to u128 for internal calculations
    let mut x: u128 = u128::from(x);
    let m: u128 = u128::from(m);
    let mut y: u128 = u128::from(y);
    let mut result: u128 = 1;

    if m == 1 {
        return 0;
    }

    while y > 0 {
        if y % 2 == 1 {
            result = (result * x) % m;
        }
        y /= 2; // the same as y = y/2
        x = x.pow(2) % m;
    }

    //convert back to u64 and immediately returning it
    u64::try_from(result).unwrap()
}

#[test]
fn test_modexp() {
    // Test for invalid input:
    // Largest prime less than 2**64.
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));

    // anything that is divisible by the base should be 0
    let num = 9; 
    assert_eq!(0, modexp(num, 3, num));
    assert_eq!(0, modexp(num, 2, num));
    assert_eq!(0, modexp(num, 5, num));
}
