/*
 * MIT License
 *
 * Copyright (c) 2022 Christophe Bismuth (christophe.bismuth@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

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

    generate_mandelbrot(
        filename,
        canvas_size,
        upper_left,
        lower_right,
        radius,
        limit,
    );
}
