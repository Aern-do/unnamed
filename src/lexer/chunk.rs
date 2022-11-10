use crate::shared::span::Span;

#[derive(Clone, Debug, Default)]
pub struct Chunk<'a> {
    pub slice: &'a str,
    pub span: Span,
}

impl<'a> Chunk<'a> {
    pub fn new(slice: &'a str, span: Span) -> Self {
        Self { slice, span }
    }
}