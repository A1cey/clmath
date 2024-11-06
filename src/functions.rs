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
}

pub const HIGHER_ORDER_FUNC_KEYWORDS: phf::Map<&'static str, HigherOrderFunc> = phf_map! {
    "Der" => HigherOrderFunc::Derivative,
    "Mod" => HigherOrderFunc::Factorial
};

/// Calculates the factorial of a 32bit unsigned integer
pub fn factorial(num: u32) -> Result<u32, Error> {
    let mut val = num;
    let mut result = 1;

    while val > 1 {
        result *= val;

        if result > u32::MAX / (val - 1) {
            return Err(Error::FactorialError(format!("The factorial of this number ({:e}) is too large to fit into the maximum range of a 32bit unsigned integer ({:e})", num, u32::MAX)));
        }

        val -= 1;
    }

    Ok(result)
}

pub fn addition(a: f64, b: f64) -> Result<f64, Error> {
    // Overflow check
    if a > f64::MAX - b {
        return Err(Error::AdditionError(format!("The sum of the two floating point numbers {} and {} is greater than the maxmimum value of a 64bit floating point number ({:e}).", a,b, f64::MAX)));
    }

    // Underflow check
    if b < 0.0 && a < f64::MIN + b.abs() {
        return Err(Error::AdditionError(format!("The sum of the two floating point numbers {} and {} is less than the minimum value of a 64bit floating point number ({:e}).", a, b, f64::MIN)));
    }

    Ok(a + b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error::FactorialError;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), Ok(1));
        assert_eq!(factorial(5), Ok(120));
        assert_eq!(factorial(500), Err(FactorialError("The factorial of this number (500) is too large to fit into the maximum range of a 32bit unsigned integer (4294967295)".to_string())))
    }

    #[test]
    fn test_addition() {
        assert_eq!(addition(5.0, 5.0), Ok(10.0));
        assert_eq!(addition(-5.0, 5.0), Ok(0.0));
        assert_eq!(addition(-5.0, -5.0), Ok(-10.0));
        assert_eq!(addition(f64::MIN, f64::MIN + 1.0), Err(Error::AdditionError(format!("The sum of the two floating point numbers {} and {} is less than the minimum value of a 64bit floating point number ({:e}).", f64::MIN, f64::MIN + 1.0, f64::MIN))));
        assert_eq!(addition(f64::MAX, f64::MAX - 1.0), Err(Error::AdditionError(format!("The sum of the two floating point numbers {} and {} is greater than the maxmimum value of a 64bit floating point number ({:e}).", f64::MAX, f64::MAX - 1.0, f64::MAX))));
    }
}
