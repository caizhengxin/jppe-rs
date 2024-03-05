use thiserror::Error as ThisError;


pub type JResult<I, O, E = Error<I>> = Result<(I, O), E>;


#[derive(Debug, PartialEq, Eq)]
pub struct Error<I> {
    pub input: I,
    pub code: ErrorKind,
}


impl<I> Error<I> {
    pub fn new(input: I, kind: ErrorKind) -> Self {
        Self { input, code: kind }
    }
}


pub trait ParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self;
}


impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Error { input, code: kind }
    }
}


#[derive(Debug, PartialEq, Eq, ThisError)]
pub enum ErrorKind {
    #[error("invalid byte length: `{offset}`")]
    InvalidByteLength {
        offset: usize,
    },
    #[error("find subsequence failure: `{offset}`")]
    SubSequence {
        offset: usize,
    },
    #[error("parse byte failure: `{offset}`")]
    Fail {
        offset: usize,
    }
}


pub fn make_error<I, E: ParseError<I>>(input: I, kind: ErrorKind) -> E {
    E::from_error_kind(input, kind)
}
