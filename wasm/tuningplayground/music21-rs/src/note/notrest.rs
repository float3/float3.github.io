use super::generalnote::GeneralNote;

pub struct NotRest {
    general_note: GeneralNote,
}

impl NotRest {
    pub fn new() -> Self {
        NotRest {
            general_note: GeneralNote::new(),
        }
    }
}
