use crate::error::ParserError;
use crate::functions::ElementaryFunc;
use crate::functions::Func;
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
    ElementaryFunction(ElementaryFunction),
    HigherOrderFunction(HigherOrderFunction),
}

#[derive(Debug)]
struct OpeningBracket;
#[derive(Debug)]
struct ClosingBracket;

#[derive(Debug)]
struct ElementaryFunction {
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

    pub fn parse(tokens: Vec<Token>) -> Result<Expression, ParserError> {
        Parser::from(tokens).expression()
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
            Expression::Empty
        };

        Ok(expression)
    }

    fn bracketed_expression(&mut self) -> Result<BracketedExpression, ParserError> {
        todo!()
    }

    fn math_expression(&mut self) -> Result<MathExpression, ParserError> {
        if let Some(token) = self.tokens.pop_front() {
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
        // lhs is already processed by now. So this somehow needs to come from the already processed side
        let expression_lhs = self.expression()?; 
        let expression_rhs = self.expression()?; 
        Ok(ElementaryFunction { expression_lhs, function, expression_rhs })
    }

    fn higher_order_function(
        &mut self,
        function: HigherOrderFunc,
    ) -> Result<HigherOrderFunction, ParserError> {
        let opening_bracket = self.opening_bracket()?;
        
        let params = (0..*function.get_param_count())
            .into_iter()
            .map(|_| self.expression())
            .collect::<Result<Vec<_>,_>>()?;
        
        let closing_bracket = self.closing_bracket()?;
        
        Ok(HigherOrderFunction {
            function,
            opening_bracket,
            params,
            closing_bracket
        })
    }

    fn opening_bracket(&mut self) -> Result<OpeningBracket, ParserError> {
        if let Some(token) = self.tokens.pop_front() {
            match token {
                Token::Symbol(Symbol::OpeningBracket) => Ok(OpeningBracket), 
                _ => return Err(ParserError::ExpectedOpeningBracket)
            }
        } else {
            Err(ParserError::ExpressionEmpty)
        }
    }
    
    fn closing_bracket(&mut self) -> Result<ClosingBracket, ParserError> {
        if let Some(token) = self.tokens.pop_front() {
            match token {
                Token::Symbol(Symbol::ClosingBracket) => Ok(ClosingBracket), 
                _ => return Err(ParserError::ExpectedClosingBracket)
            }
        } else {
            Err(ParserError::ExpressionEmpty)
        }
    }
}
