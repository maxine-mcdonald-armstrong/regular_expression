#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod annotator;
mod dfa;
mod lexer;
mod parser;

#[cfg(test)]
mod tests;

/// Represents an error during the creation of the DFA.
#[derive(Debug, PartialEq)]
pub enum DfaGenerationError {
    /// Represents an error during lexing of the input regular expression and alphabet
    /// into a token stream.
    Lexical(lexer::LexicalError),
    /// Represents an error during parsing of the token stream to an AST.
    Syntactic(parser::SyntacticError),
    /// Represents an error during annotation of the AST.
    Annotation(annotator::AnnotationError),
}

impl From<lexer::LexicalError> for DfaGenerationError {
    fn from(value: lexer::LexicalError) -> Self {
        DfaGenerationError::Lexical(value)
    }
}

impl From<parser::SyntacticError> for DfaGenerationError {
    fn from(value: parser::SyntacticError) -> Self {
        DfaGenerationError::Syntactic(value)
    }
}

impl From<annotator::AnnotationError> for DfaGenerationError {
    fn from(value: annotator::AnnotationError) -> Self {
        DfaGenerationError::Annotation(value)
    }
}

/// Generates a DFA from an input regular expression string and alphabet.
pub fn generate_dfa(raw_expression: &str, alphabet: &str) -> Result<dfa::Dfa, DfaGenerationError> {
    let sanitised_alphabet = lexer::generate_token_map(alphabet)?;
    let lexed_expression = lexer::lex_string(&sanitised_alphabet, raw_expression)?;
    let parsed_expression = parser::parse(lexed_expression)?;
    let annotated_expression = annotator::annotate_ast(parsed_expression)?;
    Ok(dfa::generate_dfa(annotated_expression))
}
