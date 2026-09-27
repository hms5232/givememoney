use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    /// Error from [`Money`](money::Money).
    MoneyError(String),
    InvalidNumber {
        /// the index of this number, starts from 0.
        position: usize,
        /// the input
        value: String,
        name: Option<String>,
    },
    /// The round is waiting for allocation.
    Unallocated,
    EmptyInput,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MoneyError(msg) => write!(f, "money error: {msg}"),
            Error::InvalidNumber {
                position,
                value,
                name,
            } => match name {
                Some(name) => write!(f, "invalid number \"{value}\": {name}"),
                None => write!(f, "invalid number \"{value}\" at {}", position + 1),
            },
            Error::Unallocated => write!(f, "this round has not been allocated yet"),
            Error::EmptyInput => write!(f, "the input is empty"),
        }
    }
}

impl std::error::Error for Error {}
