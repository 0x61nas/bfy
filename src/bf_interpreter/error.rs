use std::fmt::{Debug, Formatter, Display};

pub struct InterpreterError {
    message: String,
    pub code: i32,
}

impl InterpreterError {
    pub fn new(message: String, code: i32) -> Self {
        Self {
            message: message.to_string(),
            code,
        }
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, code: {}", self.message, self.code)
    }
}

impl std::error::Error for InterpreterError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub enum InterpreterErrorKind {
    PointerOutOfBounds(usize),
    // takes pointer value
    ValueOutOfBounds,
    ByteReadError(std::io::Error),
    ReadError,
    UnmatchedClosingBracket(usize), // takes position
}

impl InterpreterErrorKind {
    pub fn to_error(&self) -> InterpreterError {
        InterpreterError::new(self.to_string(), self.code())
    }

    fn code(&self) -> i32 {
        match self {
            InterpreterErrorKind::PointerOutOfBounds(_) => 11,
            InterpreterErrorKind::ValueOutOfBounds => 12,
            InterpreterErrorKind::ByteReadError(_) => 13,
            InterpreterErrorKind::ReadError => 14,
            InterpreterErrorKind::UnmatchedClosingBracket(_) => 15,
        }
    }
}

impl Display for InterpreterErrorKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InterpreterErrorKind::PointerOutOfBounds(pointer) => write!(f, "Pointer out of bounds {}", pointer),
            InterpreterErrorKind::ValueOutOfBounds => write!(f, "Value out of bounds"),
            InterpreterErrorKind::ByteReadError(error) =>
                write!(f, "Failed to read byte from stdin: no bytes available: {}", error),
            InterpreterErrorKind::ReadError => write!(f, "Failed to read byte from stdin: no bytes available"),
            InterpreterErrorKind::UnmatchedClosingBracket(pos) => write!(f, "Unmatched closing bracket at position {}", pos),
        }
    }
}
