use crate::parser::{
    expression::{Expression, Literal, Operator},
    primitive::{Float, Identifier, Integer},
};

pub fn eval(expression: Expression) -> f64 {
    match expression {
        Expression::Literal(Literal::Integer(Integer(integer))) => {
            integer.parse::<i32>().unwrap() as f64
        }
        Expression::Literal(Literal::Float(Float(float))) => float.parse().unwrap(),
        Expression::Literal(Literal::Identifier(Identifier(ident))) => match ident {
            "pi" => 3.14,
            _ => panic!("Unknown variable"),
        },

        Expression::Infix { lhs, operator, rhs } => {
            let lhs = eval(*lhs);
            let rhs = eval(*rhs);
            match operator {
                Operator::Plus => lhs + rhs,
                Operator::Minus => lhs - rhs,
                Operator::Multiply => lhs * rhs,
                Operator::Division => lhs / rhs,
            }
        }
        Expression::Call { ident, arguments } => {
            match ident.0 {
                "cos" => {
                    let argument = eval(arguments.elements.into_iter().next().unwrap());
                    argument.cos()
                },
                _ => panic!("unknown function")
            }
        },
    }
}
