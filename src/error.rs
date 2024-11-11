use crate::parser::Rule;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContactBookParserError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),
}
