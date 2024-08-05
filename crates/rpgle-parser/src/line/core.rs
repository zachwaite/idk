use crate::diagnostic::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LineParsingError {
    #[error("This is a failure...")]
    Unreachable,
}
