use derive_macro::Parse;

use super::{
    delimited::{Braced, Parenthesized},
    expression::Expression,
    primitive::{Colon, Comma, FuncKw, Identifier, RightBrace, RightParenthesis, Semicolon},
    punctuated::Punctuated,
};

#[derive(Parse, Debug, Clone, PartialEq, Eq)]
pub struct Argument<'source> {
    pub ident: Identifier<'source>,
    _colon: Colon,
    pub ty: Identifier<'source>,
}

#[derive(Parse, Debug, Clone, PartialEq, Eq)]
pub struct Function<'source> {
    pub func_kw: FuncKw,
    pub identifier: Identifier<'source>,
    pub arguments:
        Parenthesized<'source, Punctuated<'source, Argument<'source>, Comma, RightParenthesis>>,
    pub colon: Option<Colon>,
    #[parse_if(colon.is_some())]
    pub return_ty: Option<Identifier<'source>>,
    pub block: Braced<'source, Punctuated<'source, Expression<'source>, Semicolon, RightBrace>>,
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{
            delimited::{Braced, Parenthesized},
            primitive::{Colon, FuncKw, Identifier},
        },
        tests,
    };

    use super::{Argument, Function};

    macro_rules! func {
        ($name: ident($($arg: ident : $ty: ident),*): $body: expr) => {
            Function { func_kw: FuncKw, identifier: Identifier(stringify!($name)), arguments: Parenthesized::new(Punctuated::new(vec![$(Argument { ident: Identifier(stringify!($arg)), _colon: Colon, ty: Identifier(stringify!($ty)) }),*])), colon: None, return_ty: None, block: $body }
        };
        ($name: ident($($arg: ident : $ty: ident),*) -> $return_ty: ident: $body: expr) => {
            Function { func_kw: FuncKw, identifier: Identifier(stringify!($name)), arguments: Parenthesized::new(Punctuated::new(vec![$(Argument { ident: Identifier(stringify!($arg)), _colon: Colon, ty: Identifier(stringify!($ty)) }),*])), colon: Some(Colon), return_ty: Some(Identifier(stringify!($return_ty))), block: $body }
        };
    }
    macro_rules! body {
        () => {
            Braced::new(Punctuated::new(vec![]))
        };
    }
    tests! {
        test_function_empty("func main() {}"): func!(main(): body!());
        test_function_one_argument("func add(a: int) {}"): func!(add(a: int): body!());
        test_function_multiple_arguments("func add(a: int, b: int) {}"): func!(add(a: int, b: int): body!());
        test_function_return_ty("func add(a: int, b: int): int {}"): func!(add(a: int, b: int) -> int: body!());
    }
}
