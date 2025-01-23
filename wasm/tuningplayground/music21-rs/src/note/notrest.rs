use crate::{base::Music21ObjectTrait, prebase::ProtoM21ObjectTrait};

use super::generalnote::{GeneralNote, GeneralNoteTrait};

pub(crate) struct NotRest {
    general_note: GeneralNote,
}

pub(crate) trait NotRestTrait: GeneralNoteTrait {
    fn new() -> Self;
}

impl NotRestTrait for NotRest {
    fn new() -> Self {
        NotRest {
            general_note: <GeneralNote as GeneralNoteTrait>::new(),
        }
    }
}

impl GeneralNoteTrait for NotRest {
    fn new() -> Self {
        <NotRest as NotRestTrait>::new()
    }
}

impl Music21ObjectTrait for NotRest {
    fn new() -> Self {
        <NotRest as GeneralNoteTrait>::new()
    }
}

impl ProtoM21ObjectTrait for NotRest {
    fn new() -> Self {
        <NotRest as Music21ObjectTrait>::new()
    }
}
