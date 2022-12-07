use super::Block;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program<'a> {
    pub functions: Vec<Function<'a>>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<Argument<'a>>,
    pub return_type: &'a str,
    pub body: Block<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argument<'a> {
    pub name: &'a str,
    pub argument_type: &'a str,
}
