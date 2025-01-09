use crate::prebase::ProtoM21Object;

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct Microtone {
    proto: ProtoM21Object,
    pub alter: f64,
}

impl Microtone {
    pub fn new(alter: f64) -> Microtone {
        Microtone {
            proto: ProtoM21Object::new(),
            alter,
        }
    }
}
