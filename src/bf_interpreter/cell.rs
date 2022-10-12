use crate::arguments::Feature;
use crate::bf_interpreter::error::{InterpreterError, InterpreterErrorKind};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Byte(u8),
    Utf8(u32),
}

impl Cell {
    pub fn set_value_utf8(&mut self, ch: char) {
        match self {
            Cell::Byte(_) => {}
            Cell::Utf8(p) => {
                *p = ch as u32;
            }
        }
    }

    pub fn set_value(&mut self, ch: char) {
        match self {
            Cell::Byte(p) => {
                *p = ch as u8;
            }
            Cell::Utf8(_) => {}
        }
    }
}

impl Cell {
    pub fn default_cell(future: &Vec<Feature>) -> Self {
        if future.contains(&Feature::AllowUtf8) {
            Cell::Utf8(0)
        } else {
            Cell::Byte(0)
        }
    }

    pub fn new(value: u32, future: &Vec<Feature>) -> Self {
        if future.contains(&Feature::AllowUtf8) {
            Cell::Utf8(value)
        } else {
            Cell::Byte(value as u8)
        }
    }

    pub fn get_value(&self) -> u8 {
        match self {
            Self::Byte(value) => *value,
            Self::Utf8(value) => *value as u8,
        }
    }

    pub fn get_value_utf8(&self) -> u32 {
        match self {
            Self::Byte(value) => *value as u32,
            Self::Utf8(value) => *value,
        }
    }

    pub fn increment(&mut self, no_reverse_value: bool) -> Result<(), InterpreterError> {
        if self.get_value_utf8() == self.max_value() && no_reverse_value {
            return Err(InterpreterErrorKind::ValueOutOfBounds.to_error())
        }
        match self {
            Self::Byte(value) => {
                if *value == 255 {
                    *value = 0;
                } else {
                    *value += 1;
                }
            }
            Self::Utf8(value) => {
                if *value == 1114111 {
                    *value = 0;
                } else {
                    *value += 1;
                }
            }
        }
        Ok(())
    }

    pub fn decrement(&mut self, no_reverse_value: bool) -> Result<(), InterpreterError> {
        if self.get_value_utf8() == 0 && no_reverse_value {
            return Err(InterpreterErrorKind::ValueOutOfBounds.to_error())
        }
        match self {
            Self::Byte(value) => {
                if *value == 0 {
                    *value = 255;
                } else {
                    *value -= 1;
                }
            }
            Self::Utf8(value) => {
                if *value == 0 {
                    *value = 1114111;
                } else {
                    *value -= 1;
                }
            }
        }
        Ok(())
    }

    pub fn max_value(&self) -> u32 {
        match self {
            Self::Byte(_) => u8::MAX as u32,
            Self::Utf8(_) => u32::MAX,
        }
    }

    pub fn to_char(&self) -> Result<char, InterpreterError> {
        let c = match self {
            Self::Byte(value) => Some(*value as char),
            Self::Utf8(value) => char::from_u32(*value)
        };

        if let Some(c) = c {
            Ok(c)
        } else {
            Err(InterpreterErrorKind::InvalidUtf8.to_error())
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(value) => write!(f, "{}", value),
            Self::Utf8(value) => write!(f, "{}", value),
        }
    }
}