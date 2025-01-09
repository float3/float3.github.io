use std::fmt;

#[derive(Debug)]
pub enum ChordException {
    CannotMatchCondition { pc: i32 },
}

impl fmt::Display for ChordException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChordException::CannotMatchCondition { pc } => {
                write!(f, "Cannot match condition for pc: {}", pc)
            }
        }
    }
}

impl std::error::Error for ChordException {}
