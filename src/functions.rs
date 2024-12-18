use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::error::{Error, ErrorType, FunctionError};
use phf_macros::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Func {
    Elementary(ElementaryFunc),
    HigherOrder(HigherOrderFunc),
}

pub const ELEMENTARY_FUNC_KEYWORDS: phf::Map<char, ElementaryFunc> = phf_map! {
    '+' => ElementaryFunc::Addition,
    '/' => ElementaryFunc::Division,
    '%' => ElementaryFunc::Modulo,
    '*' => ElementaryFunc::Multiplication,
    '-' => ElementaryFunc::Subtraction,
    '<' => ElementaryFunc::LessThan,
    '>' => ElementaryFunc::GreaterThan,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ElementaryFunc {
    Addition,
    Division,
    Modulo,
    Multiplication,
    Subtraction,
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HigherOrderFunc {
    Derivative,
    Factorial,
    EucleadianModulo,
    Minimum,
    Maximum,
    Absolute,
}

pub const HIGHER_ORDER_FUNC_KEYWORDS: phf::Map<&'static str, HigherOrderFunc> = phf_map! {
    "Der" => HigherOrderFunc::Derivative,
    "Fac" => HigherOrderFunc::Factorial,
    "EMod" => HigherOrderFunc::EucleadianModulo,
    "Min" => HigherOrderFunc::Minimum,
    "Max" => HigherOrderFunc::Maximum,
    "Abs" => HigherOrderFunc::Absolute
};

/// Returns true if the first number of the two provided 64bit floating point numbers is smaller than the second else false
pub fn less_than(a: f64, b: f64) -> bool {
    a < b
}

/// Returns true if the first number of the two provided 64bit floating point numbers is greater than the second else false
pub fn greater_than(a: f64, b: f64) -> bool {
    a > b
}

/// Returns the absolute value of the provided 64bit floating point number
pub fn absolute(num: f64) -> f64 {
    num.abs()
}

/// Returns the minimum of the two provided 64bit floating point numbers
pub fn minimum(a: f64, b: f64) -> f64 {
    a.min(b)
}

/// Returns the maximum of the two provided 64bit floating point numbers
pub fn maximum(a: f64, b: f64) -> f64 {
    a.max(b)
}

/// Calculates the factorial of a 32bit unsigned integer
pub fn factorial(num: u32) -> Result<u32, Error> {
    let mut val = num;
    let mut result = 1;

    while val > 1 {
        result *= val;

        if result > u32::MAX / (val - 1) {
            return Err(create_error(
                FunctionError::FactorialError,
                num.into(),
                None,
                None,
            ));
        }

        val -= 1;
    }

    Ok(result)
}

/// Calculates the sum of two 64bit floating point numbers
pub fn addition(a: f64, b: f64) -> Result<f64, Error> {
    match a.add(b) {
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionError::UnderflowInf,
            a,
            Some(b),
            Some("addition"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionError::OverflowInf,
            a,
            Some(b),
            Some("addition"),
        )),
        result => Ok(result),
    }
}

/// Calculates the difference of two 64bit floating point numbers
pub fn subtraction(a: f64, b: f64) -> Result<f64, Error> {
    match a.sub(b) {
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionError::UnderflowInf,
            a,
            Some(b),
            Some("subtraction"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionError::OverflowInf,
            a,
            Some(b),
            Some("subtraction"),
        )),
        result => Ok(result),
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
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionError::UnderflowInf,
            a,
            Some(b),
            Some("multiplication"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionError::OverflowInf,
            a,
            Some(b),
            Some("multiplication"),
        )),
        result => Ok(result),
    }
}

/// Calculates the quotient of two 64bit floating point numbers
pub fn division(a: f64, b: f64) -> Result<f64, Error> {
    match a.div(b) {
        result if result.is_nan() => Err(create_error(
            FunctionError::DivisionByZero,
            a,
            Some(b),
            None,
        )),
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionError::UnderflowInf,
            a,
            Some(b),
            Some("division"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionError::OverflowInf,
            a,
            Some(b),
            Some("division"),
        )),
        result => Ok(result),
    }
}

fn create_error(
    error_type: FunctionError,
    first_num: f64,
    second_num: Option<f64>,
    operation_name: Option<&str>,
) -> Error {
    match error_type {
        FunctionError::DivisionByZero => {
            Error::new(
                if first_num > 10000.0 || first_num < 0.00001 {
                    format!("You cannot divide by zero. You tried to divide {:e} by {} which has no result.", first_num, second_num.unwrap())
                } else {
                    format!("You cannot divide by zero. You tried to divide {} by {} which has no result.", first_num, second_num.unwrap())
                },
                ErrorType::Func(FunctionError::DivisionByZero),
                None,
                None,
            )
        }
        FunctionError::FactorialError => Error::new(
            if first_num > 10000.0 {
                format!("The factorial of this number ({:e}) is too large to fit into the maximum range of a 32bit unsigned integer ({:e}).", first_num, u32::MAX)
            } else {
                format!("The factorial of this number ({}) is too large to fit into the maximum range of a 32bit unsigned integer ({:e}).", first_num, u32::MAX)
            },
            ErrorType::Func(FunctionError::FactorialError),
            None,
            None,
        ),
        FunctionError::OverflowInf | FunctionError::UnderflowInf => {
            let mut error_message = format!("The {} of ", operation_name.unwrap());
            if first_num > 10000.0 || first_num < 0.00001 {
                error_message.push_str(format!("{:e}", first_num).as_str());
            } else {
                error_message.push_str(format!("{}", first_num).as_str());
            }
            error_message.push_str(" and ");
            if second_num.unwrap() > 10000.0 || second_num.unwrap() < 0.00001 {
                error_message.push_str(format!("{:e}", second_num.unwrap()).as_str());
            } else {
                error_message.push_str(format!("{}", second_num.unwrap()).as_str());
            }
            if error_type == FunctionError::OverflowInf {
                error_message.push_str(format!(" results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}.", f64::MAX, f64::INFINITY).as_str());
            } else {
                error_message.push_str(format!(" results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}.", f64::MIN, f64::NEG_INFINITY).as_str());
            }

            Error::new(
                error_message,
                ErrorType::Func(error_type),
                None,
                None,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), Ok(1));
        assert_eq!(factorial(5), Ok(120));
        assert_eq!(factorial(500), Err(Error::new(
            "The factorial of this number (500) is too large to fit into the maximum range of a 32bit unsigned integer (4.294967295e9).".to_string(),
            ErrorType::Func(FunctionError::FactorialError),
            None,
            None
        )));
    }

    #[test]
    fn test_addition() {
        assert_eq!(addition(5.0, 5.0), Ok(10.0));
        assert_eq!(addition(-5.0, 5.0), Ok(0.0));
        assert_eq!(addition(-5.0, -5.0), Ok(-10.0));
        assert_eq!(addition(f64::MIN, f64::MIN + 1.0),  Err(Error::new(
            format!("The addition of {:e} and {:e} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}.",f64::MIN, f64::MIN +1.0, f64::MIN, f64::NEG_INFINITY),
            ErrorType::Func(FunctionError::UnderflowInf),
            None,
            None
        )));
        assert_eq!(addition(f64::MAX, f64::MAX - 1.0), Err(Error::new(
            format!("The addition of {:e} and {:e} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}.",f64::MAX, f64::MAX - 1.0, f64::MAX, f64::INFINITY),
            ErrorType::Func(FunctionError::OverflowInf),
            None,
            None
        )));
    }
}
