use crate::parser::{
    expression::{Expression, Operator},
    primitive::Integer,
};

pub fn eval(expression: Expression) -> i32 {
    match expression {
        Expression::Integer(Integer(integer)) => integer.parse().unwrap(),
        Expression::Infix { lhs, operator, rhs } => {
            let lhs = eval(*lhs);
            let rhs = eval(*rhs);
            match operator {
                Operator::Plus(..) => lhs + rhs,
                Operator::Minus(..) => lhs - rhs,
                Operator::Multiply(..) => lhs * rhs,
                Operator::Division(..) => lhs / rhs,
            }
        }
    }
}
