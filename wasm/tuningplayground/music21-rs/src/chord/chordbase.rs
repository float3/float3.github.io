use crate::note::generalnote::{GeneralNote, GeneralNoteTrait};

#[derive(Debug)]
pub(crate) struct ChordBase {
    general_note: GeneralNote,
}

impl ChordBase {
    pub(crate) fn new() -> Self {
        ChordBase {
            general_note: GeneralNote::new(),
        }
    }
}
