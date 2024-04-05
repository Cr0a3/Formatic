use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectError {
    DeclWithoutSymbol,
    UnknownFunction(String),
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            ObjectError::DeclWithoutSymbol => "",
            ObjectError::UnknownFunction(n) => "",
        };

        write!(f, "{}", str)
    }
}

impl std::error::Error for ObjectError {}