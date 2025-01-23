use crate::prebase::{ProtoM21Object, ProtoM21ObjectTrait};

#[derive(Debug)]
pub(crate) struct Music21Object {
    proto: ProtoM21Object,
}

pub(crate) trait Music21ObjectTrait: ProtoM21ObjectTrait {
    fn new() -> Self;
}

impl Music21ObjectTrait for Music21Object {
    fn new() -> Self {
        Music21Object {
            proto: ProtoM21Object::new(),
        }
    }
}

impl ProtoM21ObjectTrait for Music21Object {
    fn new() -> Self {
        <Music21Object as Music21ObjectTrait>::new()
    }
}
