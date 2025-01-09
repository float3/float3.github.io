use crate::base::Music21Object;

pub struct GeneralNote {
    music21: Music21Object,
}

impl GeneralNote {
    pub fn new() -> Self {
        GeneralNote {
            music21: Music21Object::new(),
        }
    }
}
