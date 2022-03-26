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

#[path = "../src/libs.rs"]
mod libs;

#[cfg(test)]
mod mandelbrot_tests {
    use num::Complex;

    use libs::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<usize>("32x64", 'x'), Some((32, 64)));
        assert_eq!(parse_pair::<f64>("4.0,8.0", ','), Some((4.0, 8.0)));
        assert_eq!(parse_pair::<f64>("4.0,8.0", 'x'), None);
        assert_eq!(parse_pair::<f64>("", 'x'), None);
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(parse_complex("32,64"), Some(Complex { re: 32.0, im: 64.0 }));
        assert_eq!(parse_complex("4.0,8.0"), Some(Complex { re: 4.0, im: 8.0 }));
        assert_eq!(parse_complex("4.0x8.0"), None);
        assert_eq!(parse_complex(""), None);
    }

    #[test]
    fn test_generate_mandelbrot() {
        generate_mandelbrot(
            "mandelbrot.png",
            (1024, 780),
            Complex { re: -1.20, im: 0.1 },
            Complex { re: -1.75, im: 0.5 },
            2.0,
            255,
        )
    }
}
