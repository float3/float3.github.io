use crate::{
    base::{Music21Object, Music21ObjectTrait},
    prebase::ProtoM21ObjectTrait,
};

#[derive(Debug)]
pub(crate) struct GeneralNote {
    music21object: Music21Object,
}

pub(crate) trait GeneralNoteTrait: Music21ObjectTrait {
    fn new() -> Self;
}

impl GeneralNoteTrait for GeneralNote {
    fn new() -> Self {
        GeneralNote {
            music21object: <Music21Object as Music21ObjectTrait>::new(),
        }
    }
}

impl Music21ObjectTrait for GeneralNote {
    fn new() -> Self {
        <GeneralNote as GeneralNoteTrait>::new()
    }
}

impl ProtoM21ObjectTrait for GeneralNote {
    fn new() -> Self {
        <GeneralNote as Music21ObjectTrait>::new()
    }
}
