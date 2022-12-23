#[derive(Clone, Debug)]
pub enum Error {
    ProcedureNotSelected,
    UnknownProcedure,
    UnknownNative,
    UnknownMarker,
}
