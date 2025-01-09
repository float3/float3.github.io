use std::fmt;

#[derive(Debug)]
pub enum PitchException {
    CannotMatchCondition { pc: i32 },
}

impl fmt::Display for PitchException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PitchException::CannotMatchCondition { pc } => {
                write!(f, "Cannot match condition for pc: {}", pc)
            }
        }
    }
}

impl std::error::Error for PitchException {}
