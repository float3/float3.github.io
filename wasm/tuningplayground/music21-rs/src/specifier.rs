#[derive(Clone, Debug)]
pub(crate) enum Specifier {
    Int(IntegerType),
    Str(String),
    Float(f64),
}
