use std::{path::Path, str::FromStr};

use image::{GrayImage, Luma};
use num::{Complex, Num, complex::Complex64};

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

/// Parse a  pair of floating-point numbers seperated by a comma as a complex number.
pub fn parse_complex<T: FromStr>(s: &str) -> Option<Complex<T>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

#[derive(Debug, Clone, Copy)]
pub struct ImageShape {
    pub width: usize,
    pub height: usize,
}

pub struct PixelPosition {
    pub x: usize,
    pub y: usize,
}

/// Given the row and column of a pixel in the output image, return the corresponding point on the
/// complex plane.
///
/// `bounds` is an object giving the width and height of the image in pixels
/// `pixel` is an object indicating a particular pixel in that image
/// `upper_left` and `lower_right` parameters are points on the complex plane designating the area
/// our image covers.
pub const fn pixel_to_point(
    bounds: ImageShape,
    pixel: PixelPosition,
    upper_left: Complex64,
    lower_right: Complex64,
) -> Complex64 {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.x as f64 * width / bounds.width as f64,

        // Pixel.x is higher for lower pixel, but the imaginary component is higher for higher
        // pixels of the image.
        im: upper_left.im - pixel.y as f64 * height / bounds.height as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives thw width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `ùpper_left` and `lower_right`arguments
/// specify points on the complex plane corresponding to the upper-left and lower-right
/// corners of the pixel buffer.
pub fn render_buffer(
    pixels: &mut [u8],
    bounds: ImageShape,
    upper_left: Complex64,
    lower_right: Complex64,
) {
    assert_eq!(pixels.len(), bounds.width * bounds.height);
    for row in 0..bounds.height {
        for column in 0..bounds.width {
            let point = pixel_to_point(
                bounds,
                PixelPosition { x: column, y: row },
                upper_left,
                lower_right,
            );
            pixels[row * bounds.width + column] =
                escape_time(point, 255).map_or(0, |count| 255 - count as u8);
        }
    }
}

pub fn render_image(
    filename: &Path,
    bounds: ImageShape,
    upper_left: Complex64,
    lower_right: Complex64,
) -> Result<(), image::ImageError> {
    GrayImage::from_par_fn(bounds.width as u32, bounds.height as u32, |column, row| {
        let point = pixel_to_point(
            bounds,
            PixelPosition {
                x: column as usize,
                y: row as usize,
            },
            upper_left,
            lower_right,
        );
        Luma([escape_time(point, 255).map_or(0, |count| 255 - count as u8)])
    })
    .save(filename)?;
    Ok(())
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

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            ImageShape {
                width: 100,
                height: 200
            },
            PixelPosition { x: 25, y: 175 },
            Complex::new(-1.0, 1.0),
            Complex::new(1.0, -1.0)
        ),
        Complex::new(-0.5, -0.75)
    );
}
