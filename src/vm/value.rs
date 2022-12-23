use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::Result;
macro_rules! impl_arithmetic {
    ($lhs: ident, $rhs: ident; $op: tt) => {{
        use crate::vm::value::Value;
        use crate::vm::error::Error;
        match ($lhs, $rhs) {
            (Value::Byte(lhs), Value::Byte(rhs)) => Ok(Value::Byte(lhs $op rhs)),
            (Value::Short(lhs), Value::Short(rhs)) => Ok(Value::Short(lhs $op rhs)),
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs $op rhs)),
            (Value::Long(lhs), Value::Long(rhs)) => Ok(Value::Long(lhs $op rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Float(lhs $op rhs)),
            (Value::Double(lhs), Value::Double(rhs)) => Ok(Value::Double(lhs $op rhs)),
            _ => Err(Error::IncompatibleValues)
        }
    }}
}
macro_rules! impl_compare {
    ($lhs: ident, $rhs: ident; $op: tt) => {{
        use crate::vm::value::Value;
        use crate::vm::error::Error;
        match ($lhs, $rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            (Value::Byte(lhs), Value::Byte(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            (Value::Short(lhs), Value::Short(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            (Value::Long(lhs), Value::Long(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            (Value::Double(lhs), Value::Double(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            _ => Err(Error::IncompatibleValues)
        }
    }}
}
macro_rules! impl_logic {
    ($lhs: ident, $rhs: ident; $op: tt) => {{
        use crate::vm::value::Value;
        use crate::vm::error::Error;
        match ($lhs, $rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs $op rhs)),
            _ => Err(Error::IncompatibleValues)
        }
    }}
}
macro_rules! from {
    ($target: ident -> $variant: ident) => {
        impl From<$target> for Value {
            fn from(target: $target) -> Value {
                Value::$variant(target)
            }
        }
    };
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

from!(bool -> Boolean);
from!(i8 -> Byte);
from!(i32 -> Integer);
from!(i16 -> Short);
from!(i64 -> Long);
from!(f32 -> Float);
from!(f64 -> Double);
impl Value {
    pub fn try_add(self, rhs: Value) -> Result<Value> {
        impl_arithmetic!(self, rhs; +)
    }
    pub fn try_sub(self, rhs: Value) -> Result<Value> {
        impl_arithmetic!(self, rhs; -)
    }
    pub fn try_mul(self, rhs: Value) -> Result<Value> {
        impl_arithmetic!(self, rhs; *)
    }
    pub fn try_div(self, rhs: Value) -> Result<Value> {
        impl_arithmetic!(self, rhs; /)
    }
    pub fn try_eq(self, rhs: Value) -> Result<Value> {
        impl_compare!(self, rhs; ==)
    }
    pub fn try_lt(self, rhs: Value) -> Result<Value> {
        impl_compare!(self, rhs; <)
    }
    pub fn try_gt(self, rhs: Value) -> Result<Value> {
        impl_compare!(self, rhs; >)
    }
    pub fn try_lteq(self, rhs: Value) -> Result<Value> {
        impl_compare!(self, rhs; <=)
    }
    pub fn try_gteq(self, rhs: Value) -> Result<Value> {
        impl_compare!(self, rhs; >=)
    }
    pub fn try_or(self, rhs: Value) -> Result<Value> {
        impl_logic!(self, rhs; ||)
    }
    pub fn try_and(self, rhs: Value) -> Result<Value> {
        impl_logic!(self, rhs; &&)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Byte(value) => write!(f, "{}", value),
            Value::Short(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Long(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Double(value) => write!(f, "{}", value),
        }
    }
}
