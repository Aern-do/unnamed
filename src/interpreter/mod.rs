use crate::parser::{
    expression::{Expression, Literal, Operator},
    primitive::{Float, Integer},
};

pub fn eval(expression: Expression) -> f64 {
    match expression {
        Expression::Literal(Literal::Integer(Integer(integer))) => {
            integer.parse::<i32>().unwrap() as f64
        }
        Expression::Literal(Literal::Float(Float(float))) => float.parse().unwrap(),
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
