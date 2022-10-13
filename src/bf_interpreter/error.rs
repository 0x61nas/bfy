use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
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
    PointerOutOfBounds(usize), // takes pointer value
    ValueOutOfBounds,
    IoError(std::io::Error),
    FlushError(std::io::Error),
    UnmatchedBracket,
    InvalidUtf8,
}

impl InterpreterErrorKind {
    pub fn to_error(&self) -> InterpreterError {
        InterpreterError::new(self.to_string(), self.code())
    }

    fn code(&self) -> i32 {
        match self {
            InterpreterErrorKind::PointerOutOfBounds(_) => 11,
            InterpreterErrorKind::ValueOutOfBounds => 12,
            InterpreterErrorKind::IoError(_) => 13,
            InterpreterErrorKind::FlushError(_) => 14,
            InterpreterErrorKind::UnmatchedBracket => 15,
            InterpreterErrorKind::InvalidUtf8 => 16,
        }
    }
}

impl Display for InterpreterErrorKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            InterpreterErrorKind::PointerOutOfBounds(pointer) => {
                write!(f, "Pointer out of bounds {}", pointer)
            }
            InterpreterErrorKind::ValueOutOfBounds => write!(f, "Value out of bounds"),
            InterpreterErrorKind::IoError(error) => write!(
                f,
                "Failed to read byte from stdin: no bytes available: {}",
                error
            ),
            InterpreterErrorKind::FlushError(e) => write!(f, "Failed to flush stdout: {}", e),
            InterpreterErrorKind::UnmatchedBracket => write!(f, "Unmatched bracket"),
            InterpreterErrorKind::InvalidUtf8 => write!(f, "Invalid utf8"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // for testing only

    #[test]
    fn test_error_kind_display() {
        let error = InterpreterErrorKind::PointerOutOfBounds(10).to_error();
        assert_eq!(error.to_string(), "Pointer out of bounds 10");
        assert_eq!(error.code, 11);

        let error = InterpreterErrorKind::ValueOutOfBounds.to_error();
        assert_eq!(error.to_string(), "Value out of bounds");
        assert_eq!(error.code, 12);

        let error =
            InterpreterErrorKind::IoError(std::io::Error::new(std::io::ErrorKind::Other, "test"))
                .to_error();
        assert_eq!(
            error.to_string(),
            "Failed to read byte from stdin: no bytes available: test"
        );
        assert_eq!(error.code, 13);

        let error = InterpreterErrorKind::UnmatchedBracket.to_error();
        assert_eq!(error.to_string(), "Unmatched bracket");
        assert_eq!(error.code, 15);

        let error = InterpreterErrorKind::InvalidUtf8.to_error();
        assert_eq!(error.to_string(), "Invalid utf8");
        assert_eq!(error.code, 16);
    }

    #[test]
    fn test_error_display() {
        let error = InterpreterError::new("test".to_string(), 10);
        assert_eq!(error.to_string(), "test");
        assert_eq!(error.code, 10);
    }

    #[test]
    fn test_error_debug() {
        let error = InterpreterError::new("test".to_string(), 10);
        assert_eq!(format!("{:?}", error), "test, code: 10");
    }
}
