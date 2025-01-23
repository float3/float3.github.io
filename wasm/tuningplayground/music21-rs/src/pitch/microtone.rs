use crate::prebase::ProtoM21Object;

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub(crate) struct Microtone {
    proto: ProtoM21Object,
    pub(crate) alter: f64,
}

impl Microtone {
    pub(crate) fn new(alter: f64) -> Microtone {
        Microtone {
            proto: ProtoM21Object::new(),
            alter,
        }
    }
}
