//! Generates an AST from a [`Token`] stream.

use crate::lexer::ReservedToken;
use crate::lexer::Token;
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter::Peekable;

#[cfg(test)]
mod tests;

/// Runtime error representing that the input token stream was invalid due to a missing token.
///
/// For example, a [`ReservedToken::Choice`] Not followed by a valid [`Expression`], or a
/// [`ReservedToken::LeftPrecedence`] without a matching [`ReservedToken::RightPrecedence`].
#[derive(Debug, PartialEq)]
struct MissingExpectedTokenError {
    actual_token: Option<Token>,
    expected_token: Token,
}

impl Display for MissingExpectedTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.actual_token {
            None => write!(f, "Parser expected {} but found None", self.expected_token),
            Some(t) => write!(f, "Parser expected {} but found {}", self.expected_token, t),
        }
    }
}

/// Runtime error representing that the input token stream was invalid due to an extra token.
///
/// For example, a [`ReservedToken::Closure`] not after a non-empty [`Expression`].
#[derive(Debug, PartialEq)]
struct UnexpectedTokenError {
    token: Token,
}

impl Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Parser received an unexpected token: {}.", self.token)
    }
}

/// Wraps all parser-based errors.
#[derive(Debug, PartialEq)]
enum SyntacticError {
    UnexpectedToken(UnexpectedTokenError),
    MissingExpectedToken(MissingExpectedTokenError),
}

/// Represents an AST node.
///
/// [`Expression::EmptyString`] and [`Expression::Char`] are always and the only leaf nodes.
#[derive(Debug, PartialEq)]
pub enum Expression {
    Concatenation(Vec<Expression>),
    Choice(Vec<Expression>),
    Closure(Box<Expression>),
    Char(char),
    EmptyString,
}

/// Parses an [`Expression::Char`] or [`Expression`] as defined in the [syntax documentation](crate).
fn parse_atomic<I>(token_stream: &mut Peekable<I>) -> Result<Expression, SyntacticError>
where
    I: Iterator<Item = Token>,
{
    match token_stream.peek() {
        Some(Token::Char(c)) => {
            let c = *c;
            token_stream.next();
            Ok(Expression::Char(c))
        }
        Some(Token::ReservedToken(ReservedToken::LeftPrecedence)) => {
            token_stream.next();
            let expression = parse_expression(token_stream)?;
            match token_stream.next() {
                Some(Token::ReservedToken(ReservedToken::RightPrecedence)) => Ok(expression),
                None => Err(SyntacticError::MissingExpectedToken(
                    MissingExpectedTokenError {
                        actual_token: None,
                        expected_token: Token::ReservedToken(ReservedToken::RightPrecedence),
                    },
                )),
                Some(t) => Err(SyntacticError::UnexpectedToken(UnexpectedTokenError {
                    token: t,
                })),
            }
        }
        _ => Ok(Expression::EmptyString),
    }
}

/// Parses an [`Expression::Closure`] as defined in the [syntax documentation](crate).
fn parse_closure<I>(token_stream: &mut Peekable<I>) -> Result<Expression, SyntacticError>
where
    I: Iterator<Item = Token>,
{
    if token_stream.peek() == Some(&Token::ReservedToken(ReservedToken::Closure)) {
        // Closure acting on empty string is disallowed
        return Err(SyntacticError::UnexpectedToken(UnexpectedTokenError {
            token: Token::ReservedToken(ReservedToken::Closure),
        }));
    }
    let atomic = parse_atomic(token_stream)?;
    match token_stream.peek() {
        Some(Token::ReservedToken(ReservedToken::Closure)) => {
            token_stream.next(); // consume the '*'
            Ok(Expression::Closure(Box::from(atomic)))
        }
        _ => Ok(atomic),
    }
}

/// Parses an [`Expression::Concatenation`] as defined in the [syntax documentation](crate).
fn parse_concatenation<I>(token_stream: &mut Peekable<I>) -> Result<Expression, SyntacticError>
where
    I: Iterator<Item = Token>,
{
    let closure = parse_closure(token_stream)?;
    if token_stream.peek().is_none()
        || token_stream.peek() == Some(&Token::ReservedToken(ReservedToken::RightPrecedence))
        || token_stream.peek() == Some(&Token::ReservedToken(ReservedToken::Choice))
    {
        return Ok(closure);
    }
    let mut concatenation: Vec<Expression> = Vec::from([closure]);
    while token_stream.peek().is_some() {
        match token_stream.peek().unwrap() {
            // end of concatenation
            Token::ReservedToken(ReservedToken::Choice)
            | Token::ReservedToken(ReservedToken::RightPrecedence) => {
                return Ok(Expression::Concatenation(concatenation));
            }
            // next closure
            Token::Char(_) | Token::ReservedToken(ReservedToken::LeftPrecedence) => {
                concatenation.push(parse_closure(token_stream)?);
            }
            // invalid
            t => {
                return Err(SyntacticError::UnexpectedToken(UnexpectedTokenError {
                    token: *t,
                }));
            }
        }
    }
    Ok(Expression::Concatenation(concatenation))
}

/// Parses an [`Expression::Choice`] as defined in the [syntax documentation](crate).
fn parse_choice<I>(token_stream: &mut Peekable<I>) -> Result<Expression, SyntacticError>
where
    I: Iterator<Item = Token>,
{
    let juxtaposition = parse_concatenation(token_stream)?;
    if token_stream.peek().is_none()
        || token_stream.peek() == Some(&Token::ReservedToken(ReservedToken::RightPrecedence))
    {
        return Ok(juxtaposition);
    }
    let mut choice: Vec<Expression> = Vec::from([juxtaposition]);
    while token_stream.peek().is_some() {
        match token_stream.peek().unwrap() {
            Token::ReservedToken(ReservedToken::Choice) => {
                token_stream.next();
                choice.push(parse_concatenation(token_stream)?);
            }
            Token::ReservedToken(ReservedToken::RightPrecedence) => {
                token_stream.next();
                break;
            }
            t => {
                return Err(SyntacticError::UnexpectedToken(UnexpectedTokenError {
                    token: *t,
                }));
            }
        }
    }
    Ok(Expression::Choice(choice))
}

/// Parses an [`Expression`] as defined in the [syntax documentation](crate).
///
/// The hierarchy made explicit in the [syntax](crate) is followed here, so [`parse_expression`]
/// matches a choice, [`parse_choice`] matches zero or more concatenations separated by "|", etc.
fn parse_expression<I>(token_stream: &mut Peekable<I>) -> Result<Expression, SyntacticError>
where
    I: Iterator<Item = Token>,
{
    parse_choice(token_stream)
}

/// Generates the root of an AST representing the token_stream.
fn parse(token_stream: Vec<Token>) -> Result<Expression, SyntacticError> {
    let mut token_stream_iterable = token_stream.into_iter().peekable();
    let expression = parse_expression(&mut token_stream_iterable)?;
    match token_stream_iterable.next() {
        None => Ok(expression),
        Some(t) => Err(SyntacticError::UnexpectedToken(UnexpectedTokenError {
            token: t,
        })),
    }
}
