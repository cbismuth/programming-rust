extern crate image;
extern crate num;

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use num::Complex;

pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None,
        },
    }
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn resolve_escape_count(c: Complex<f64>, limit: u8, radius: f64) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > radius * radius {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn map_pixel_to_point(
    pixel: (usize, usize),
    canvas_size: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let factors: (f64, f64) = (
        (lower_right.re - upper_left.re) / (canvas_size.0 as f64),
        (lower_right.im - upper_left.im) / (canvas_size.1 as f64),
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * factors.0,
        im: upper_left.im - pixel.1 as f64 * factors.1,
    }
}

pub fn render(
    pixels: &mut [u8],
    canvas_size: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    radius: f64,
    limit: u8,
) {
    assert_eq!(pixels.len(), canvas_size.0 * canvas_size.1);

    for row in 0..canvas_size.1 {
        for column in 0..canvas_size.0 {
            let index = row * canvas_size.0 + column;

            let point = map_pixel_to_point((row, column), canvas_size, upper_left, lower_right);

            pixels[index] = match resolve_escape_count(point, limit, radius) {
                Some(count) => limit - count,
                None => 0,
            }
        }
    }
}

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    canvas_size: (usize, usize),
) -> Result<(), Error> {
    let file = File::create(filename)?;
    let encoder = PngEncoder::new(file);
    match encoder.write_image(
        &pixels,
        canvas_size.0 as u32,
        canvas_size.1 as u32,
        ColorType::L8,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}
