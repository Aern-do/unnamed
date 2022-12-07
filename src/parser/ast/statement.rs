use super::{Block, BoxedNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    While {
        test: BoxedNode<'a>,
        body: Block<'a>,
    },
    Let {
        mutable: bool,
        name: &'a str,
        init: Option<BoxedNode<'a>>,
    },
    If {
        test: BoxedNode<'a>,
        consequent: Block<'a>,
        alternate: Option<Alternate<'a>>,
    },
    Return {
        expression: BoxedNode<'a>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Alternate<'a> {
    If(BoxedNode<'a>),
    End(Block<'a>),
}
