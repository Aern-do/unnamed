use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
impl Add for Span {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Span::new(self.start, rhs.end)
    }
}
macro_rules! from_range {
    ($type: ident) => {
        impl From<std::ops::Range<$type>> for Span {
            fn from(range: std::ops::Range<$type>) -> Self {
                Span::new(range.start as usize, range.end as usize)
            }
        }
    };
    ($($target: ident),+ $(,)?) => {
        $(from_range!($target);)+
    };
}
from_range!(i8, u8, i16, u16, i32, u32, i64, u64, isize);
