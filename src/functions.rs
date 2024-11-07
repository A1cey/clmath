use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::error::Error;
use phf_macros::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Func {
    Elementary(ElementaryFunc),
    HigherOrder(HigherOrderFunc),
}

pub const ELEMENTARY_FUNC_KEYWORDS: phf::Map<&'static str, ElementaryFunc> = phf_map! {
    "+" => ElementaryFunc::Addition,
    "/" => ElementaryFunc::Division,
    "%" => ElementaryFunc::Modulo,
    "*" => ElementaryFunc::Multiplication,
    "-" => ElementaryFunc::Subtraction,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ElementaryFunc {
    Addition,
    Division,
    Modulo,
    Multiplication,
    Subtraction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HigherOrderFunc {
    Derivative,
    Factorial,
    EucleadianModulo,
}

pub const HIGHER_ORDER_FUNC_KEYWORDS: phf::Map<&'static str, HigherOrderFunc> = phf_map! {
    "Der" => HigherOrderFunc::Derivative,
    "Fac" => HigherOrderFunc::Factorial,
    "EMod" => HigherOrderFunc::EucleadianModulo
};

/// Calculates the factorial of a 32bit unsigned integer
pub fn factorial(num: u32) -> Result<u32, Error> {
    let mut val = num;
    let mut result = 1;

    while val > 1 {
        result *= val;

        if result > u32::MAX / (val - 1) {
            return Err(Error::FactorialError(
                format!("The factorial of this number ({:e}) is too large to fit into the maximum range of a 32bit unsigned integer ({:e})", num, u32::MAX)
            ));
        }

        val -= 1;
    }

    Ok(result)
}

/// Calculates the sum of two 64bit floating point numbers
pub fn addition(a: f64, b: f64) -> Result<f64, Error> {
    match a.add(b) {
        result if  result.is_sign_negative() && result.is_infinite() => Err(Error::UnderflowInf(
            format!("The addition of {a} and {b} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MIN, f64::NEG_INFINITY)
        )),
        result if result.is_infinite() => Err(Error::OverflowInf(
            format!("The addition of {a} and {b} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MAX, f64::INFINITY)
        )),
        result => Ok(result) ,
    }
}

/// Calculates the difference of two 64bit floating point numbers
pub fn subtraction(a: f64, b: f64) -> Result<f64, Error> {
    match a.sub(b) {
        result if  result.is_sign_negative() && result.is_infinite() => Err(Error::UnderflowInf(
            format!("The subtraction of {a} and {b} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MIN, f64::NEG_INFINITY)
        )),
        result if result.is_infinite() => Err(Error::OverflowInf(
            format!("The subtraction of {a} and {b} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MAX, f64::INFINITY)
        )),
        result => Ok(result) ,
    }
}

/// Calculates the modulus of the division of two 64bit floating point numbers
pub fn modulo(a: f64, n: f64) -> f64 {
    a.rem(n)
}

/// Calculates the euclidean modulus of the division of two 64bit floating point numbers
pub fn modulo_euclid(a: f64, n: f64) -> f64 {
    a.rem_euclid(n)
}

/// Calculates the product of two 64bit floating point numbers
pub fn multiplication(a: f64, b: f64) -> Result<f64, Error> {
    match a.mul(b) {
        result if  result.is_sign_negative() && result.is_infinite() => Err(Error::UnderflowInf(
            format!("The multiplication of {a} and {b} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MIN, f64::NEG_INFINITY)
        )),
        result if result.is_infinite() => Err(Error::OverflowInf(
            format!("The multiplication of {a} and {b} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MAX, f64::INFINITY)
        )),
        result => Ok(result) ,
    }
}

/// Calculates the quotient of two 64bit floating point numbers
pub fn division(a: f64, b: f64) -> Result<f64, Error> {
    match a.div(b) {
        result if result.is_nan() => Err(Error::DivisionByZero(format!(
            "You cannot divide by zero. You tried to divide {a} by {b} which has no result."
        ))),
        result if  result.is_sign_negative() && result.is_infinite() => Err(Error::UnderflowInf(
            format!("The division of {a} and {b} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MIN, f64::NEG_INFINITY)
        )),
        result if result.is_infinite() => Err(Error::OverflowInf(
            format!("The division of {a} and {b} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}", f64::MAX, f64::INFINITY)
        )),
        result => Ok(result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error::FactorialError;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), Ok(1));
        assert_eq!(factorial(5), Ok(120));
        assert_eq!(factorial(500), Err(FactorialError(
            "The factorial of this number (500) is too large to fit into the maximum range of a 32bit unsigned integer (4294967295)".to_string()
        ))
    )
    }

    #[test]
    fn test_addition() {
        assert_eq!(addition(5.0, 5.0), Ok(10.0));
        assert_eq!(addition(-5.0, 5.0), Ok(0.0));
        assert_eq!(addition(-5.0, -5.0), Ok(-10.0));
        assert_eq!(addition(f64::MIN, f64::MIN + 1.0),  Err(Error::UnderflowInf(
            format!("This addition of {} and {} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}",f64::MIN, f64::MIN +1.0, f64::MIN, f64::NEG_INFINITY)
        )));
        assert_eq!(addition(f64::MAX, f64::MAX - 1.0), Err(Error::OverflowInf(
            format!("This addition of {} and {} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}",f64::MAX, f64::MAX - 1.0, f64::MAX, f64::INFINITY)
        )));
    }
}
