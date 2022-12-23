#[derive(Clone, Debug)]
pub enum Error {
    Underflow,
    IncompatibleValues,
    UnknownNative,
}
