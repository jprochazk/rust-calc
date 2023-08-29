use super::op::Op;
use crate::expr::BinaryOp;
use crate::expr::Expr;
use crate::expr::UnaryOp;

pub type Bytecode = Vec<Op>;

pub fn compile(expr: &Expr) -> Bytecode {
    fn emit(expr: &Expr, ops: &mut Bytecode) {
        match expr {
            Expr::Binary(expr) => {
                emit(&expr.left, ops);
                emit(&expr.right, ops);
                let op = match expr.op {
                    BinaryOp::Add => Op::BAdd,
                    BinaryOp::Sub => Op::BSub,
                    BinaryOp::Mul => Op::BMul,
                    BinaryOp::Div => Op::BDiv,
                };
                ops.push(op);
            }
            Expr::Unary(expr) => {
                emit(&expr.right, ops);
                let op = match expr.op {
                    UnaryOp::Plus => return,
                    UnaryOp::Minus => Op::UMinus,
                };
                ops.push(op);
            }
            Expr::Int(value) => ops.push(Op::LInt(*value)),
        }
    }

    let mut ops = Vec::new();
    emit(expr, &mut ops);
    ops
}
