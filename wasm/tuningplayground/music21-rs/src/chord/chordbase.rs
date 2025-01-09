use crate::note::generalnote::GeneralNote;

pub struct ChordBase {
    general_note: GeneralNote,
}

impl ChordBase {
    pub fn new() -> Self {
        ChordBase {
            general_note: GeneralNote::new(),
        }
    }
}
