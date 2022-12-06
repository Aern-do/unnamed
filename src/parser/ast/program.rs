use super::Block;

#[derive(Clone, Debug)]
pub struct Program<'a> {
    pub functions: Vec<Function<'a>>,
}
#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<Argument<'a>>,
    pub return_type: &'a str,
    pub body: Block<'a>,
}
#[derive(Clone, Debug)]
pub struct Argument<'a> {
    pub name: &'a str,
    pub argument_type: &'a str,
}
