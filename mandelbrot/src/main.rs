extern crate image;
extern crate num;

use std::env;
use std::str::FromStr;

use libs::*;

mod libs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        eprintln!(
            "Usage: {} <filename> <canvas_size> <upper_left> <lower_right> <radius> <limit>",
            args[0]
        );
        eprintln!(
            "Example: {} mandelbrot.png 1024x780 -1.20,0.1 -1.75,0.5 2.0 255",
            args[0]
        );
        std::process::exit(1);
    }

    let filename = &args[1];
    let canvas_size: (usize, usize) = parse_pair(&args[2], 'x').expect("Error parsing canvas size");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left complex point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right complex point");
    let radius = f64::from_str(&args[5]).expect("Error parsing radius");
    let limit = u8::from_str(&args[6]).expect("Error parsing limit");

    let mut pixels = vec![0 as u8; canvas_size.0 * canvas_size.1];

    render(
        &mut pixels,
        canvas_size,
        upper_left,
        lower_right,
        radius,
        limit,
    );

    write_image(filename, &pixels, canvas_size).expect("Error writing image")
}
