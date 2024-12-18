use std::str::FromStr;

use num::{Complex, Num};

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of iterations it took for `C`
/// to leave the circle of radius 2 centered on the origin.
/// If `c` seems to be a member (more precisely, if we reached the iteration limit without begin
/// able to prove that `c` is not a member), return `None`
pub fn escape_time<T: Default + Copy + Num + PartialOrd + From<f32>>(
    c: Complex<T>,
    limit: usize,
) -> Option<usize> {
    let mut z = Complex::<T>::default();
    let four = T::from(4.0);
    (0..limit).find(|_| {
        if z.norm_sqr() > four {
            true
        } else {
            z = z * z + c;
            false
        }
    })
}

/// Oarse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse correctly, return `None`
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    s.split_once(separator)
        .and_then(|(left, right)| Some((left.parse::<T>().ok()?, right.parse::<T>().ok()?)))
}

pub fn parse_complex<T: FromStr>(s: &str) -> Option<Complex<T>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("20,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    use num::complex::Complex64;
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex64 {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex::<f64>(",-0.0625"), None);
}
