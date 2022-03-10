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
