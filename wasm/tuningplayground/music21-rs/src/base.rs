use crate::prebase::ProtoM21Object;

pub struct Music21Object {
    proto: ProtoM21Object,
}

impl Music21Object {
    pub fn new() -> Self {
        Music21Object {
            proto: ProtoM21Object::new(),
        }
    }
}
