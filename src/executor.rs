use crate::{
    error::{CLMathError, FunctionError},
    functions::{
        absolute, addition, division, factorial, greater_than, less_than, maximum, minimum, modulo,
        modulo_euclid, multiplication, subtraction, ElementaryFunc, FunctionReturnType,
        HigherOrderFunc,
    },
    parser::{
        ElementaryFunction, Expression, Function, HigherOrderFunction, MathExpression, Start,
    },
};

pub fn execute(expression: Start) -> Result<Option<FunctionReturnType>, CLMathError> {
    match expression {
        Start::Expression(expr) => Ok(Some(
            execute_expression(expr).map_err(CLMathError::Function)?,
        )),
        Start::Empty => Ok(None),
    }
}

fn execute_expression(expression: Expression) -> Result<FunctionReturnType, FunctionError> {
    match expression {
        Expression::MathExpression(expr) => execute_math_expression(*expr),
        Expression::BracketedExpression(expr) => execute_expression(expr.expression),
    }
}

fn execute_math_expression(
    expression: MathExpression,
) -> Result<FunctionReturnType, FunctionError> {
    match expression {
        MathExpression::Function(function) => execute_function(function),
        MathExpression::Number(num) => Ok(FunctionReturnType::F64(num)),
        MathExpression::Variable(var) => {
            Ok(var.value.map_or(FunctionReturnType::Str(var.name), |num| {
                FunctionReturnType::F64(num)
            }))
        }
    }
}

fn execute_function(function: Function) -> Result<FunctionReturnType, FunctionError> {
    match function {
        Function::ElementaryFunction(func) => run_elementary_function(func),
        Function::HigherOrderFunction(func) => run_higher_order_function(func),
    }
}

fn run_elementary_function(
    function: ElementaryFunction,
) -> Result<FunctionReturnType, FunctionError> {
    let lhs = execute_expression(function.expression_lhs)?.get_f64()?;
    let rhs = execute_expression(function.expression_rhs)?.get_f64()?;

    match function.function {
        ElementaryFunc::Addition => addition(lhs, rhs),
        ElementaryFunc::Division => division(lhs, rhs),
        ElementaryFunc::Modulo => Ok(modulo(lhs, rhs)),
        ElementaryFunc::Multiplication => multiplication(lhs, rhs),
        ElementaryFunc::Subtraction => subtraction(lhs, rhs),
        ElementaryFunc::LessThan => Ok(less_than(lhs, rhs)),
        ElementaryFunc::GreaterThan => Ok(greater_than(lhs, rhs)),
    }
}

fn run_higher_order_function(
    function: HigherOrderFunction,
) -> Result<FunctionReturnType, FunctionError> {
    let first_param = execute_expression(function.params.expression)?;
    let mut other_params = function
        .params
        .expression_comma
        .map_or(Ok(vec![]), |params| {
            params
                .into_iter()
                .map(|(_, expr)| execute_expression(expr))
                .collect::<Result<Vec<_>, _>>()
        })?;

    match function.function {
        HigherOrderFunc::Factorial(_) => factorial(first_param.get_u32()?),
        HigherOrderFunc::EucleadianModulo(_) => Ok(modulo_euclid(
            first_param.get_f64()?,
            other_params
                .pop()
                .expect("There should be enough parameters after parsing.")
                .get_f64()?,
        )),
        HigherOrderFunc::Minimum(_) => Ok(minimum(
            first_param.get_f64()?,
            other_params
                .pop()
                .expect("There should be enough parameters after parsing.")
                .get_f64()?,
        )),
        HigherOrderFunc::Maximum(_) => Ok(maximum(
            first_param.get_f64()?,
            other_params
                .pop()
                .expect("There should be enough parameters after parsing.")
                .get_f64()?,
        )),
        HigherOrderFunc::Absolute(_) => Ok(absolute(first_param.get_f64()?)),
    }
}
