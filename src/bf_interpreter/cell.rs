use crate::arguments::Feature;
use crate::bf_interpreter::error::{InterpreterError, InterpreterErrorKind};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Byte(u8),
    Utf8(u32),
}

impl Cell {
    pub fn set_value(&mut self, ch: char) {
        match self {
            Cell::Byte(p) => {
                *p = ch as u8;
            }
            Cell::Utf8(p) => {
                *p = ch as u32;
            }
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

    #[allow(dead_code)]
    /// For testing purposes
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
            return Err(InterpreterErrorKind::ValueOutOfBounds.to_error());
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
            return Err(InterpreterErrorKind::ValueOutOfBounds.to_error());
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
            Self::Utf8(_) => 1114111,
        }
    }

    pub fn to_char(&self) -> Result<char, InterpreterError> {
        let c = match self {
            Self::Byte(value) => Some(*value as char),
            Self::Utf8(value) => char::from_u32(*value),
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_increment_u8_no_revers() {
        let mut cell = Cell::default_cell(&vec![]);
        cell.increment(true).unwrap();
        assert_eq!(cell, Cell::Byte(1));

        for _ in 0..254 {
            cell.increment(true).unwrap();
        }
        assert_eq!(cell, Cell::Byte(255));

        assert_eq!(
            cell.increment(true).unwrap_err(),
            InterpreterErrorKind::ValueOutOfBounds.to_error()
        );
        assert_eq!(cell, Cell::Byte(255));
    }

    #[test]
    fn test_increment_u32_no_revers() {
        let mut cell = Cell::default_cell(&vec![Feature::AllowUtf8]);
        cell.increment(true).unwrap();
        assert_eq!(cell, Cell::Utf8(1));

        for _ in 0..1114110 {
            cell.increment(true).unwrap();
        }
        assert_eq!(cell, Cell::Utf8(1114111));

        assert_eq!(
            cell.increment(true).unwrap_err(),
            InterpreterErrorKind::ValueOutOfBounds.to_error()
        );
        assert_eq!(cell, Cell::Utf8(1114111));
    }

    #[test]
    fn test_increment_u8_revers() {
        let mut cell = Cell::default_cell(&vec![]);
        cell.increment(false).unwrap();
        assert_eq!(cell, Cell::Byte(1));

        for _ in 0..254 {
            cell.increment(false).unwrap();
        }
        assert_eq!(cell, Cell::Byte(255));

        cell.increment(false).unwrap();
        assert_eq!(cell, Cell::Byte(0));
    }

    #[test]
    fn test_increment_u32_revers() {
        let mut cell = Cell::default_cell(&vec![Feature::AllowUtf8]);
        cell.increment(false).unwrap();
        assert_eq!(cell, Cell::Utf8(1));

        for _ in 0..1114110 {
            cell.increment(false).unwrap();
        }
        assert_eq!(cell, Cell::Utf8(1114111));

        cell.increment(false).unwrap();
        assert_eq!(cell, Cell::Utf8(0));
    }

    #[test]
    fn test_decrement_u8_no_revers() {
        let mut cell = Cell::new(255, &vec![]);
        cell.decrement(true).unwrap();
        assert_eq!(cell, Cell::Byte(254));

        for _ in 0..254 {
            cell.decrement(true).unwrap();
        }
        assert_eq!(cell, Cell::Byte(0));

        assert_eq!(
            cell.decrement(true).unwrap_err(),
            InterpreterErrorKind::ValueOutOfBounds.to_error()
        );
        assert_eq!(cell, Cell::Byte(0));
    }

    #[test]
    fn test_decrement_u32_no_revers() {
        let mut cell = Cell::new(1114111, &vec![Feature::AllowUtf8]);
        cell.decrement(true).unwrap();
        assert_eq!(cell, Cell::Utf8(1114110));

        for _ in 0..1114110 {
            cell.decrement(true).unwrap();
        }
        assert_eq!(cell, Cell::Utf8(0));

        assert_eq!(
            cell.decrement(true).unwrap_err(),
            InterpreterErrorKind::ValueOutOfBounds.to_error()
        );
        assert_eq!(cell, Cell::Utf8(0));
    }

    #[test]
    fn test_decrement_u8_revers() {
        let mut cell = Cell::new(0, &vec![]);
        cell.decrement(false).unwrap();
        assert_eq!(cell, Cell::Byte(255));

        for _ in 0..254 {
            cell.decrement(false).unwrap();
        }
        assert_eq!(cell, Cell::Byte(1));

        cell.decrement(false).unwrap();
        assert_eq!(cell, Cell::Byte(0));
    }

    #[test]
    fn test_decrement_u32_revers() {
        let mut cell = Cell::new(0, &vec![Feature::AllowUtf8]);
        cell.decrement(false).unwrap();
        assert_eq!(cell, Cell::Utf8(1114111));

        for _ in 0..1114110 {
            cell.decrement(false).unwrap();
        }
        assert_eq!(cell, Cell::Utf8(1));

        cell.decrement(false).unwrap();
        assert_eq!(cell, Cell::Utf8(0));
    }

    #[test]
    fn test_to_char() {
        let cell = Cell::new(65, &vec![]);
        assert_eq!(cell.to_char().unwrap(), 'A');

        let cell = Cell::new(129408, &vec![Feature::AllowUtf8]);
        assert_eq!(cell.to_char().unwrap(), '🦀');

        let cell = Cell::new(129392, &vec![Feature::AllowUtf8]);
        assert_eq!(cell.to_char().unwrap(), '🥰');
    }

    #[test]
    fn test_set_value() {
        let mut cell = Cell::default_cell(&vec![]);
        cell.set_value('A');
        assert_eq!(cell, Cell::Byte(65));

        let mut cell = Cell::default_cell(&vec![Feature::AllowUtf8]);
        cell.set_value('🦀');
        assert_eq!(cell, Cell::Utf8(129408));
    }
}
