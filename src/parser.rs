use crate::error::Error;
use crate::functions::Func;
use crate::tokenizer::Token;

pub fn parse(tokens: &Vec<Token>) -> Result<String, Error> {
    Ok(format!("{:?}", tokens))
}

enum ExpressionOption {
    Number(Number),
    Variable(Variable),
    Function(Function),
}

struct Expression {
    value: ExpressionOption,
}

struct Number {
    number: f64,
}

struct Variable {
    name: String,
    value: Option<f64>,
}

struct Function {
    function: Func,
    params: Vec<Expression>,
}
