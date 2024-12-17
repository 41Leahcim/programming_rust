use std::{ops::RemAssign, time::Instant};

fn gcd<T: Default + PartialEq + PartialOrd + RemAssign<T> + Clone>(mut left: T, mut right: T) -> T {
    let zero = T::default();
    assert!(left != zero && right != zero);
    while right != zero {
        if right < left {
            (left, right) = (right, left);
        }
        right %= left.clone();
    }
    left
}

fn main() {
    const MAXIMUM: u64 = 6_000;
    let start = Instant::now();
    let mut max = 0;
    for left in 1..MAXIMUM {
        for right in 1..left {
            let gcd = gcd(left, right);
            if gcd > max {
                println!("{gcd}");
                max = gcd;
            }
        }
    }
    println!("{:?}", start.elapsed());
}
