use std::env::args;

pub const fn gcd(mut left: u64, mut right: u64) -> u64 {
    assert!(left != 0 && right != 0);
    while right != 0 {
        if right < left {
            (left, right) = (right, left);
        }
        right %= left;
    }
    left
}

fn main() {
    let numbers = args()
        .skip(1)
        .map(|arg| {
            arg.trim()
                .parse::<u64>()
                .expect("Error parsing argument, only pass numbers as arguments")
        })
        .collect::<Vec<_>>();

    assert!(!numbers.is_empty(), "Usage: gcd NUMBER ...");

    let divider = numbers.iter().skip(1).copied().fold(numbers[0], gcd);
    println!("The greatest common divisor of {numbers:?} is {divider}");
}
