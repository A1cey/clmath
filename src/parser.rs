use crate::error::Error;
use crate::functions::ElementaryFunc;
use crate::functions::HigherOrderFunc;
use crate::tokenizer::Symbol;
use crate::tokenizer::Token;
use crate::tokenizer::Variable;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Expression {
    BracketedExpression(Box<BracketedExpression>),
    MathExpression(Box<MathExpression>),
    Empty,
}

#[derive(Debug)]
struct BracketedExpression {
    opening_bracket: OpeningBracket,
    expression: Expression,
    closing_bracket: ClosingBracket,
}

#[derive(Debug)]
enum MathExpression {
    Number(f64),
    Variable(Variable),
    Function(Function),
}

#[derive(Debug)]
enum Function {
    LowerOrderFunction(LowerOrderFunction),
    HigherOrderFunction(HigherOrderFunction),
}

#[derive(Debug)]
struct OpeningBracket {}
#[derive(Debug)]
struct ClosingBracket {}

#[derive(Debug)]
struct LowerOrderFunction {
    expression_lhs: Expression,
    function: ElementaryFunc,
    expression_rhs: Expression,
}

#[derive(Debug)]
struct HigherOrderFunction {
    function: HigherOrderFunc,
    opening_bracket: OpeningBracket,
    params: Vec<Expression>,
    closing_bracket: ClosingBracket,
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    fn from(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into(),
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Expression, Vec<Error>> {
        Parser::from(tokens)
            .expression()
            .map_err(|err| vec![Error::new("Error while parsing.".into(), err, None, None)])
    }

    fn expression(&mut self) -> Result<Expression, Box<dyn std::error::Error>> {
        let expression = if let Some(first_token) = self.tokens.front() {
            match first_token {
                Token::Symbol(Symbol::OpeningBracket) => {
                    Expression::BracketedExpression(Box::new(self.bracketed_expression()?))
                }
                _ => Expression::MathExpression(Box::new(self.math_expression()?)),
            }
        } else {
            Expression::Empty
        };

        Ok(expression)
    }

    fn bracketed_expression(&mut self) -> Result<BracketedExpression, Box<dyn std::error::Error>> {
        todo!()
    }

    fn math_expression(&mut self) -> Result<MathExpression, Box<dyn std::error::Error>> {
        todo!()
    }
}
