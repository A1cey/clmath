#![allow(dead_code)]

use crate::error::CLMathError;
use crate::error::ParserError;
use crate::functions::ElementaryFunc;
use crate::functions::Func;
use crate::functions::HigherOrderFunc;
use crate::tokenizer::Symbol;
use crate::tokenizer::Token;
use crate::tokenizer::Variable;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Start {
    Expression(Expression),
    Empty,
}

#[derive(Debug)]
pub enum Expression {
    BracketedExpression(Box<BracketedExpression>),
    MathExpression(Box<MathExpression>),
}

#[derive(Debug)]
pub struct BracketedExpression {
    opening_bracket: OpeningBracket,
    pub expression: Expression,
    closing_bracket: ClosingBracket,
}

#[derive(Debug)]
pub enum MathExpression {
    Number(f64),
    Variable(Variable),
    Function(Function),
}

#[derive(Debug)]
pub enum Function {
    ElementaryFunction(ElementaryFunction),
    HigherOrderFunction(HigherOrderFunction),
}

#[derive(Debug)]
pub struct OpeningBracket;
#[derive(Debug)]
pub struct ClosingBracket;
#[derive(Debug)]
pub struct Comma;

#[derive(Debug)]
pub struct ElementaryFunction {
    pub expression_lhs: Expression,
    pub function: ElementaryFunc,
    pub expression_rhs: Expression,
}

#[derive(Debug)]
pub struct HigherOrderFunction {
    pub function: HigherOrderFunc,
    opening_bracket: OpeningBracket,
    pub params: Params,
    closing_bracket: ClosingBracket,
}

#[derive(Debug)]
pub struct Params {
    pub expression: Expression,
    pub expression_comma: Option<Vec<(Comma, Expression)>>,
}

impl Params {
    fn new(expression: Expression, expression_comma: Option<Vec<(Comma, Expression)>>) -> Self {
        Self {
            expression,
            expression_comma,
        }
    }
}

pub struct Parser {
    tokens: VecDeque<Token>,
    lhs_queue: VecDeque<Expression>,
    expression_ends_queue: VecDeque<usize>,
    returning: bool,
    curr_idx: usize,
}

impl Parser {
    fn from(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into(),
            lhs_queue: VecDeque::new(),
            expression_ends_queue: VecDeque::new(),
            returning: true,
            curr_idx: 0,
        }
    }

    fn pop(&mut self) -> Option<Token> {
        self.curr_idx += 1;
        self.tokens.pop_front()
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Start, CLMathError> {
        Parser::from(tokens).start().map_err(CLMathError::Parser)
    }

    fn start(&mut self) -> Result<Start, ParserError> {
        if self.tokens.is_empty() {
            Ok(Start::Empty)
        } else {
            Ok(Start::Expression(self.expression()?))
        }
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        let expression = if let Some(first_token) = self.tokens.front() {
            match first_token {
                Token::Symbol(Symbol::OpeningBracket) => {
                    Expression::BracketedExpression(Box::new(self.bracketed_expression()?))
                }
                _ => Expression::MathExpression(Box::new(self.math_expression()?)),
            }
        } else {
            panic!("Queue cannot be empty. It was checked before to be not empty.")
        };

        if !self.tokens.is_empty() && self.returning {
            if matches!(
                self.tokens.front(),
                Some(Token::Function(Func::Elementary(_)))
            ) {
                self.lhs_queue.push_front(expression);
                self.expression()
            } else if self.expression_ends_queue.is_empty() {
                Err(ParserError::ExpectedElementaryFunction)
            } else {
                Ok(expression)
            }
        } else {
            Ok(expression)
        }
    }

    fn expression_until(&mut self) -> Result<Expression, ParserError> {
        let expression = if let Some(first_token) = self.tokens.front() {
            match first_token {
                Token::Symbol(Symbol::OpeningBracket) => {
                    Expression::BracketedExpression(Box::new(self.bracketed_expression()?))
                }
                _ => Expression::MathExpression(Box::new(self.math_expression()?)),
            }
        } else {
            panic!("Queue cannot be empty. It was checked before to be not empty.")
        };

        let end = *self.expression_ends_queue.front().expect(
            "Should be a value there. This function is only called after value is pushed in queue.",
        );

        if self.curr_idx < end && !self.tokens.is_empty() {
            self.lhs_queue.push_front(expression);
            if matches!(
                self.tokens.front(),
                Some(Token::Function(Func::Elementary(_)))
            ) {
                self.expression_until()
            } else {
                Err(ParserError::ExpectedElementaryFunction)
            }
        } else {
            Ok(expression)
        }
    }

    fn bracketed_expression(&mut self) -> Result<BracketedExpression, ParserError> {
        let opening_bracket = self.opening_bracket()?;

        let mut idx = 0;
        let mut opening_brackets = 1;

        loop {
            match self.tokens.get(idx) {
                Some(Token::Symbol(Symbol::OpeningBracket)) => opening_brackets += 1,
                Some(Token::Symbol(Symbol::ClosingBracket)) => opening_brackets -= 1,
                Some(_) => (),
                None => return Err(ParserError::ExpectedClosingBracket),
            };

            if opening_brackets == 0 {
                break;
            } else {
                idx += 1;
            }
        }

        self.expression_ends_queue.push_front(idx + self.curr_idx); // Offset from start

        let expression = self.expression_until()?;

        self.expression_ends_queue.pop_front();

        let closing_bracket = self.closing_bracket()?;

        Ok(BracketedExpression {
            opening_bracket,
            expression,
            closing_bracket,
        })
    }

    fn math_expression(&mut self) -> Result<MathExpression, ParserError> {
        if let Some(token) = self.pop() {
            let expr = match token {
                Token::Number(num) => MathExpression::Number(num),
                Token::Variable(var) => MathExpression::Variable(var),
                Token::Function(func) => MathExpression::Function(self.function(func)?),
                _ => return Err(ParserError::ExpectedMathExpression),
            };

            Ok(expr)
        } else {
            Err(ParserError::ExpressionEmpty)
        }
    }

    fn function(&mut self, function: Func) -> Result<Function, ParserError> {
        let f = match function {
            Func::Elementary(func) => Function::ElementaryFunction(self.elementary_function(func)?),
            Func::HigherOrder(func) => {
                Function::HigherOrderFunction(self.higher_order_function(func)?)
            }
        };

        Ok(f)
    }

    fn elementary_function(
        &mut self,
        function: ElementaryFunc,
    ) -> Result<ElementaryFunction, ParserError> {
        if let Some(expression_lhs) = self.lhs_queue.pop_front() {
            let expression_rhs = self.non_returning_expression()?; // ensures left to right execution of expression
            Ok(ElementaryFunction {
                expression_lhs,
                function,
                expression_rhs,
            })
        } else {
            Err(ParserError::NoLhsExpressionProvided)
        }
    }

    fn higher_order_function(
        &mut self,
        function: HigherOrderFunc,
    ) -> Result<HigherOrderFunction, ParserError> {
        let opening_bracket = self.opening_bracket()?;

        let params = self.params(&function)?;

        let closing_bracket = self.closing_bracket()?;

        Ok(HigherOrderFunction {
            function,
            opening_bracket,
            params,
            closing_bracket,
        })
    }

    fn opening_bracket(&mut self) -> Result<OpeningBracket, ParserError> {
        if let Some(token) = self.pop() {
            match token {
                Token::Symbol(Symbol::OpeningBracket) => Ok(OpeningBracket),
                _ => Err(ParserError::ExpectedOpeningBracket),
            }
        } else {
            Err(ParserError::ExpressionEmpty)
        }
    }

    fn closing_bracket(&mut self) -> Result<ClosingBracket, ParserError> {
        if let Some(token) = self.pop() {
            match token {
                Token::Symbol(Symbol::ClosingBracket) => Ok(ClosingBracket),
                _ => Err(ParserError::ExpectedClosingBracket),
            }
        } else {
            Err(ParserError::ExpressionEmpty)
        }
    }

    fn non_returning_expression(&mut self) -> Result<Expression, ParserError> {
        self.returning = false;
        let expression = self.expression()?;
        self.returning = true;

        Ok(expression)
    }

    fn params(&mut self, function_type: &HigherOrderFunc) -> Result<Params, ParserError> {
        let param_count = function_type.get_param_count();

        let expression = self.non_returning_expression()?;

        if *param_count > 1 {
            let mut expression_comma = Vec::new();

            for _ in 1..*param_count {
                let comma = self.comma()?;

                let expression = self.non_returning_expression()?;
                expression_comma.push((comma, expression))
            }

            return Ok(Params::new(expression, Some(expression_comma)));
        }

        Ok(Params::new(expression, None))
    }

    fn comma(&mut self) -> Result<Comma, ParserError> {
        if matches!(self.pop(), Some(Token::Symbol(Symbol::Comma))) {
            Ok(Comma)
        } else {
            Err(ParserError::ExpectedComma)
        }
    }
}
