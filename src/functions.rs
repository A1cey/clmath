use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::error::{FunctionError, FunctionErrorType};
use phf_macros::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Func {
    Elementary(ElementaryFunc),
    HigherOrder(HigherOrderFunc),
}

#[derive(Debug)]
pub enum FunctionReturnType {
    F64(f64),
    Bool(bool),
    U32(u32),
    Str(String),
}

impl Display for FunctionReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FunctionReturnType::F64(num) => num.to_string(),
            FunctionReturnType::U32(num) => num.to_string(),
            FunctionReturnType::Bool(val) => val.to_string(),
            FunctionReturnType::Str(s) => s.into(),
        };

        write!(f, "{s}")
    }
}

impl FunctionReturnType {
    pub fn get_f64(self) -> Result<f64, FunctionError> {
        match self {
            FunctionReturnType::F64(num) => Ok(num),
            FunctionReturnType::U32(num) => Err(FunctionError::new(
                format!("Expected a f64 as a parameter but got a u32 {:?}", num),
                FunctionErrorType::InvalidParameterType,
            )),
            FunctionReturnType::Bool(val) => Err(FunctionError::new(
                format!("Expected a f64 as a parameter but got a boolean {:?}", val),
                FunctionErrorType::InvalidParameterType,
            )),
            FunctionReturnType::Str(s) => Err(FunctionError::new(
                format!("Expected a f64 as a parameter but got a string {:?}", s),
                FunctionErrorType::InvalidParameterType,
            )),
        }
    }

    pub fn get_u32(self) -> Result<u32, FunctionError> {
        match self {
            FunctionReturnType::F64(num) => {
                if num.fract() == 0.0 {
                    Ok(num as u32)
                } else {
                    Err(FunctionError::new(
                        format!("Expected a u32 as a parameter but got a f64 {:?}", num),
                        FunctionErrorType::InvalidParameterType,
                    ))
                }
            }
            FunctionReturnType::U32(num) => Ok(num),
            FunctionReturnType::Bool(val) => Err(FunctionError::new(
                format!("Expected a u32 as a parameter but got a boolean {:?}", val),
                FunctionErrorType::InvalidParameterType,
            )),
            FunctionReturnType::Str(s) => Err(FunctionError::new(
                format!("Expected a u32 as a parameter but got a string {:?}", s),
                FunctionErrorType::InvalidParameterType,
            )),
        }
    }
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
    Factorial(u8),
    EucleadianModulo(u8),
    Minimum(u8),
    Maximum(u8),
    Absolute(u8),
}

impl HigherOrderFunc {
    pub fn get_param_count(&self) -> &u8 {
        match self {
            HigherOrderFunc::Absolute(num_of_params)
            | HigherOrderFunc::Factorial(num_of_params)
            | HigherOrderFunc::Maximum(num_of_params)
            | HigherOrderFunc::Minimum(num_of_params)
            | HigherOrderFunc::EucleadianModulo(num_of_params) => num_of_params,
        }
    }
}

pub const HIGHER_ORDER_FUNC_KEYWORDS: phf::Map<&'static str, HigherOrderFunc> = phf_map! {
    "Fac" => HigherOrderFunc::Factorial( 1),
    "EMod" => HigherOrderFunc::EucleadianModulo( 2),
    "Min" => HigherOrderFunc::Minimum( 2),
    "Max" => HigherOrderFunc::Maximum( 2),
    "Abs" => HigherOrderFunc::Absolute( 1)
};

/// Returns true if the first number of the two provided 64bit floating point numbers is smaller than the second else false
pub fn less_than(a: f64, b: f64) -> FunctionReturnType {
    FunctionReturnType::Bool(a < b)
}

/// Returns true if the first number of the two provided 64bit floating point numbers is greater than the second else false
pub fn greater_than(a: f64, b: f64) -> FunctionReturnType {
    FunctionReturnType::Bool(a > b)
}

/// Returns the absolute value of the provided 64bit floating point number
pub fn absolute(num: f64) -> FunctionReturnType {
    FunctionReturnType::F64(num.abs())
}

/// Returns the minimum of the two provided 64bit floating point numbers
pub fn minimum(a: f64, b: f64) -> FunctionReturnType {
    FunctionReturnType::F64(a.min(b))
}

/// Returns the maximum of the two provided 64bit floating point numbers
pub fn maximum(a: f64, b: f64) -> FunctionReturnType {
    FunctionReturnType::F64(a.max(b))
}

/// Calculates the factorial of a 32bit unsigned integer
pub fn factorial(num: u32) -> Result<FunctionReturnType, FunctionError> {
    let mut val = num;
    let mut result = 1;

    while val > 1 {
        result *= val;

        if result > u32::MAX / (val - 1) {
            return Err(create_error(
                FunctionErrorType::FactorialError,
                num.into(),
                None,
                None,
            ));
        }

        val -= 1;
    }

    Ok(FunctionReturnType::U32(result))
}

/// Calculates the sum of two 64bit floating point numbers
pub fn addition(a: f64, b: f64) -> Result<FunctionReturnType, FunctionError> {
    match a.add(b) {
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionErrorType::UnderflowInf,
            a,
            Some(b),
            Some("addition"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionErrorType::OverflowInf,
            a,
            Some(b),
            Some("addition"),
        )),
        result => Ok(FunctionReturnType::F64(result)),
    }
}

/// Calculates the difference of two 64bit floating point numbers
pub fn subtraction(a: f64, b: f64) -> Result<FunctionReturnType, FunctionError> {
    match a.sub(b) {
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionErrorType::UnderflowInf,
            a,
            Some(b),
            Some("subtraction"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionErrorType::OverflowInf,
            a,
            Some(b),
            Some("subtraction"),
        )),
        result => Ok(FunctionReturnType::F64(result)),
    }
}

/// Calculates the modulus of the division of two 64bit floating point numbers
pub fn modulo(a: f64, n: f64) -> FunctionReturnType {
    FunctionReturnType::F64(a.rem(n))
}

/// Calculates the euclidean modulus of the division of two 64bit floating point numbers
pub fn modulo_euclid(a: f64, n: f64) -> FunctionReturnType {
    FunctionReturnType::F64(a.rem_euclid(n))
}

/// Calculates the product of two 64bit floating point numbers
pub fn multiplication(a: f64, b: f64) -> Result<FunctionReturnType, FunctionError> {
    match a.mul(b) {
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionErrorType::UnderflowInf,
            a,
            Some(b),
            Some("multiplication"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionErrorType::OverflowInf,
            a,
            Some(b),
            Some("multiplication"),
        )),
        result => Ok(FunctionReturnType::F64(result)),
    }
}

/// Calculates the quotient of two 64bit floating point numbers
pub fn division(a: f64, b: f64) -> Result<FunctionReturnType, FunctionError> {
    match a.div(b) {
        result if result.is_nan() => Err(create_error(
            FunctionErrorType::DivisionByZero,
            a,
            Some(b),
            None,
        )),
        result if result.is_sign_negative() && result.is_infinite() => Err(create_error(
            FunctionErrorType::UnderflowInf,
            a,
            Some(b),
            Some("division"),
        )),
        result if result.is_infinite() => Err(create_error(
            FunctionErrorType::OverflowInf,
            a,
            Some(b),
            Some("division"),
        )),
        result => Ok(FunctionReturnType::F64(result)),
    }
}

fn create_error(
    error_type: FunctionErrorType,
    first_num: f64,
    second_num: Option<f64>,
    operation_name: Option<&str>,
) -> FunctionError {
    match error_type {
        FunctionErrorType::DivisionByZero => {
            FunctionError::new(
                if !(0.00001..=10000.0).contains(&first_num) {
                    format!("You cannot divide by zero. You tried to divide {:e} by {} which has no result.", first_num, second_num.unwrap())
                } else {
                    format!("You cannot divide by zero. You tried to divide {} by {} which has no result.", first_num, second_num.unwrap())
                },
                FunctionErrorType::DivisionByZero,
            )
        }
        FunctionErrorType::FactorialError => FunctionError::new(
            if first_num > 10000.0 {
                format!("The factorial of this number ({:e}) is too large to fit into the maximum range of a 32bit unsigned integer ({:e}).", first_num, u32::MAX)
            } else {
                format!("The factorial of this number ({}) is too large to fit into the maximum range of a 32bit unsigned integer ({:e}).", first_num, u32::MAX)
            },
            FunctionErrorType::FactorialError,
        ),
        FunctionErrorType::OverflowInf | FunctionErrorType::UnderflowInf => {
            let mut error_message = format!("The {} of ", operation_name.unwrap());
            if !(0.00001..=10000.0).contains(&first_num) {
                error_message.push_str(format!("{:e}", first_num).as_str());
            } else {
                error_message.push_str(format!("{}", first_num).as_str());
            }
            error_message.push_str(" and ");
            if !(0.00001..=10000.0).contains(&second_num.unwrap()) {
                error_message.push_str(format!("{:e}", second_num.unwrap()).as_str());
            } else {
                error_message.push_str(format!("{}", second_num.unwrap()).as_str());
            }
            if matches!(error_type, FunctionErrorType::OverflowInf) {
                error_message.push_str(format!(" results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}.", f64::MAX, f64::INFINITY).as_str());
            } else {
                error_message.push_str(format!(" results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}.", f64::MIN, f64::NEG_INFINITY).as_str());
            }

            FunctionError::new(error_message, error_type)
        }
        FunctionErrorType::InvalidParameterType => {
            panic!("Not a error from a math function.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap().get_u32().unwrap(), 1);
        assert_eq!(factorial(5).unwrap().get_u32().unwrap(), 120);
        let err = factorial(500).unwrap_err();
        assert_eq!(err.error, "The factorial of this number (500) is too large to fit into the maximum range of a 32bit unsigned integer (4.294967295e9).".to_string());
    }

    #[test]
    fn test_addition() {
        assert_eq!(addition(5.0, 5.0).unwrap().get_f64().unwrap(), 10.0);
        assert_eq!(addition(-5.0, 5.0).unwrap().get_f64().unwrap(), 0.0);
        assert_eq!(addition(-5.0, -5.0).unwrap().get_f64().unwrap(), -10.0);

        let err = addition(f64::MIN, f64::MIN + 1.0).unwrap_err();
        assert_eq!(err.error, format!("The addition of {:e} and {:e} results in an underflow of the 64bit floating point range ({:e}) and can only be displayed as {}.",f64::MIN, f64::MIN +1.0, f64::MIN, f64::NEG_INFINITY));

        let err = addition(f64::MAX, f64::MAX - 1.0).unwrap_err();
        assert_eq!(err.error, format!("The addition of {:e} and {:e} results in an overflow of the 64bit floating point range ({:e}) and can only be displayed as {}.",f64::MAX, f64::MAX - 1.0, f64::MAX, f64::INFINITY));
    }
}
