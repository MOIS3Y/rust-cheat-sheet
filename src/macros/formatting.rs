//! Format specifiers for printing macros.
//!
//! Demonstrates format specifiers used in `println!`, `format!`, etc.
//! Syntax: `{[argument][:[fill][align][sign][#][0][width].[precision]][type]}`

/// Basic format specifier `{}`.
///
/// Default formatting for most types.
pub fn format_basic() {
    let x = 42;
    println!("Default: {}", x);

    let s = "hello";
    println!("String: {}", s);
}

/// Debug format `{:?}`.
///
/// Shows debug representation.
pub fn format_debug() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };
    println!("Debug: {:?}", p);
}

/// Pretty debug format `{:#?}`.
///
/// Multi-line formatted debug output.
pub fn format_debug_pretty() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Config {
        name: String,
        values: Vec<i32>,
    }

    let c = Config {
        name: String::from("test"),
        values: vec![1, 2, 3],
    };
    println!("Pretty debug:\n{:#?}", c);
}

/// Display trait `{}`.
///
/// User-friendly display format.
pub fn format_display() {
    use std::fmt;

    struct Celsius(f64);

    impl fmt::Display for Celsius {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}°C", self.0)
        }
    }

    let temp = Celsius(23.5);
    println!("Display: {}", temp);
}

/// Binary format `{:b}`.
///
/// Shows number in binary.
pub fn format_binary() {
    let n = 42;
    println!("Binary: {:b}", n);
}

/// Octal format `{:o}`.
///
/// Shows number in octal.
pub fn format_octal() {
    let n = 42;
    println!("Octal: {:o}", n);
}

/// Hexadecimal format `{:x}` and `{:X}`.
///
/// Lowercase and uppercase hex.
pub fn format_hex() {
    let n = 255;
    println!("Hex lower: {:x}", n);
    println!("Hex upper: {:X}", n);
}

/// Hex with prefix `{:#x}`.
///
/// Adds `0x` prefix.
pub fn format_hex_prefix() {
    let n = 255;
    println!("Hex with prefix: {:#x}", n);
    println!("Hex upper with prefix: {:#X}", n);
}

/// Floating point default `{:}`.
///
/// Default float formatting.
pub fn format_float_default() {
    let pi = 3.14159265359;
    println!("Float default: {}", pi);
}

/// Floating point precision `{:.2}`.
///
/// Specifies decimal places.
pub fn format_float_precision() {
    let pi = 3.14159265359;
    println!("2 decimals: {:.2}", pi);
    println!("4 decimals: {:.4}", pi);
    println!("0 decimals: {:.0}", pi);
}

/// Floating point width and precision `{:8.2}`.
///
/// Minimum width with precision.
pub fn format_float_width_precision() {
    let pi = 3.14159265359;
    println!("Width 8: {:8.2}", pi);
    println!("Width 10: {:10.2}", pi);
}

/// Scientific notation `{:e}`.
///
/// Exponential format.
pub fn format_scientific() {
    let n = 1234567.0;
    println!("Scientific: {:e}", n);
    println!("Scientific upper: {:E}", n);
}

/// Pointer format `{:p}`.
///
/// Shows memory address.
pub fn format_pointer() {
    let x = 42;
    let ptr = &x as *const i32;
    println!("Pointer: {:p}", ptr);
}

/// Boolean format `{}`.
///
/// Prints `true` or `false`.
pub fn format_boolean() {
    println!("Boolean: {}", true);
    println!("Boolean: {}", false);
}

/// Character format `{}`.
///
/// Prints single character.
pub fn format_char() {
    let c = 'A';
    println!("Char: {}", c);
}

/// Escaped character `{:?}`.
///
/// Shows escape sequences.
pub fn format_char_escaped() {
    let newline = '\n';
    let tab = '\t';
    println!("Escaped: {:?}", newline);
    println!("Escaped: {:?}", tab);
}

/// String slice `{}`.
///
/// Prints string content.
pub fn format_str() {
    let s = "hello";
    println!("String: {}", s);
}

/// String debug `{:?}`.
///
/// Shows quotes and escapes.
pub fn format_str_debug() {
    let s = "hello\nworld";
    println!("Debug: {:?}", s);
}

/// Alignment left `{:<width}`.
///
/// Left-align within width.
pub fn format_align_left() {
    println!("Left: |{:<10}|", "hello");
}

/// Alignment right `{:>width}`.
///
/// Right-align within width.
pub fn format_align_right() {
    println!("Right: |{:>10}|", "hello");
}

/// Alignment center `{:^width}`.
///
/// Center within width.
pub fn format_align_center() {
    println!("Center: |{:^10}|", "hello");
}

/// Fill character `{:_<width}`.
///
/// Custom fill character.
pub fn format_fill() {
    println!("Fill left: |{:_<10}|", "hello");
    println!("Fill right: |{:_>10}|", "hello");
    println!("Fill center: |{:_^10}|", "hello");
}

/// Sign for positive `{:+}`.
///
/// Always show sign.
pub fn format_sign_always() {
    println!("Positive: {:+}", 42);
    println!("Negative: {:+}", -42);
}

/// Sign minus only `{: }`.
///
/// Space for positive, minus for negative.
pub fn format_sign_space() {
    println!("Positive: {: }", 42);
    println!("Negative: {: }", -42);
}

/// Zero padding `{:0width}`.
///
/// Pad with zeros.
pub fn format_zero_pad() {
    let n = 42;
    println!("Zero pad: {:05}", n);
    println!("Zero pad: {:08}", n);
}

/// Width for integers `{:width}`.
///
/// Minimum field width.
pub fn format_width() {
    let n = 42;
    println!("Width 5: |{:5}|", n);
    println!("Width 10: |{:10}|", n);
}

/// Alternate hex with padding.
///
/// Combines `#` and `0`.
pub fn format_hex_padded() {
    let n = 255;
    println!("Hex padded: {:#08x}", n);
}

/// Format multiple values.
///
/// Multiple format specifiers.
pub fn format_multiple() {
    let name = "Alice";
    let age = 30;
    let score = 95.5;
    println!("Name: {}, Age: {:3}, Score: {:.1}", name, age, score);
}

/// Dynamic width `{:<width$}`.
///
/// Width from parameter.
pub fn format_dynamic_width() {
    let s = "hello";
    let width = 10;
    println!("Dynamic: |{:<width$}|", s, width = width);
}

/// Dynamic precision `{:.precision$}`.
///
/// Precision from parameter.
pub fn format_dynamic_precision() {
    let pi = 3.14159265359;
    let precision = 3;
    println!("Dynamic: {:.precision$}", pi, precision = precision);
}

/// Format without newline `print!`.
///
/// Same specifiers work with `print!`.
pub fn format_without_newline() {
    let x = 42;
    print!("No newline: {} ", x);
    print!("Continued: {}", x + 1);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic() {
        format_basic();
    }

    #[test]
    fn test_format_debug() {
        format_debug();
    }

    #[test]
    fn test_format_debug_pretty() {
        format_debug_pretty();
    }

    #[test]
    fn test_format_display() {
        format_display();
    }

    #[test]
    fn test_format_binary() {
        format_binary();
    }

    #[test]
    fn test_format_octal() {
        format_octal();
    }

    #[test]
    fn test_format_hex() {
        format_hex();
    }

    #[test]
    fn test_format_hex_prefix() {
        format_hex_prefix();
    }

    #[test]
    fn test_format_float_default() {
        format_float_default();
    }

    #[test]
    fn test_format_float_precision() {
        format_float_precision();
    }

    #[test]
    fn test_format_float_width_precision() {
        format_float_width_precision();
    }

    #[test]
    fn test_format_scientific() {
        format_scientific();
    }

    #[test]
    fn test_format_pointer() {
        format_pointer();
    }

    #[test]
    fn test_format_boolean() {
        format_boolean();
    }

    #[test]
    fn test_format_char() {
        format_char();
    }

    #[test]
    fn test_format_char_escaped() {
        format_char_escaped();
    }

    #[test]
    fn test_format_str() {
        format_str();
    }

    #[test]
    fn test_format_str_debug() {
        format_str_debug();
    }

    #[test]
    fn test_format_align_left() {
        format_align_left();
    }

    #[test]
    fn test_format_align_right() {
        format_align_right();
    }

    #[test]
    fn test_format_align_center() {
        format_align_center();
    }

    #[test]
    fn test_format_fill() {
        format_fill();
    }

    #[test]
    fn test_format_sign_always() {
        format_sign_always();
    }

    #[test]
    fn test_format_sign_space() {
        format_sign_space();
    }

    #[test]
    fn test_format_zero_pad() {
        format_zero_pad();
    }

    #[test]
    fn test_format_width() {
        format_width();
    }

    #[test]
    fn test_format_hex_padded() {
        format_hex_padded();
    }

    #[test]
    fn test_format_multiple() {
        format_multiple();
    }

    #[test]
    fn test_format_dynamic_width() {
        format_dynamic_width();
    }

    #[test]
    fn test_format_dynamic_precision() {
        format_dynamic_precision();
    }

    #[test]
    fn test_format_without_newline() {
        format_without_newline();
    }
}
