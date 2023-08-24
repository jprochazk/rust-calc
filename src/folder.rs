use crate::expr::BinaryOp;
use crate::expr::Expr;
use crate::expr::UnaryOp;

pub fn fold(expr: &Expr) -> i64 {
    match expr {
        Expr::Binary(expr) => {
            let left = fold(&expr.left);
            let right = fold(&expr.right);
            match expr.op {
                BinaryOp::Add => left + right,
                BinaryOp::Sub => left - right,
                BinaryOp::Mul => left * right,
                BinaryOp::Div => left / right,
            }
        }
        Expr::Unary(expr) => {
            let right = fold(&expr.right);
            match expr.op {
                UnaryOp::Plus => right,
                UnaryOp::Minus => -right,
            }
        }
        Expr::Int(value) => *value,
    }
}
