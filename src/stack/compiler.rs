use super::op::Op;
use crate::expr::BinaryOp;
use crate::expr::Expr;
use crate::expr::UnaryOp;

pub type ConstPool = Vec<i64>;
pub type Bytecode = Vec<Op>;

const MIN_INLINE_INT: i64 = i16::MIN as i64;
const MAX_INLINE_INT: i64 = i16::MAX as i64;

pub fn compile(expr: &Expr) -> (Bytecode, ConstPool) {
    fn emit(expr: &Expr, ops: &mut Bytecode, pool: &mut ConstPool) {
        match expr {
            Expr::Binary(expr) => {
                emit(&expr.left, ops, pool);
                emit(&expr.right, ops, pool);
                match expr.op {
                    BinaryOp::Add => ops.push(Op::BAdd),
                    BinaryOp::Sub => ops.push(Op::BSub),
                    BinaryOp::Mul => ops.push(Op::BMul),
                    BinaryOp::Div => ops.push(Op::BDiv),
                }
            }
            Expr::Unary(expr) => {
                emit(&expr.right, ops, pool);
                match expr.op {
                    UnaryOp::Plus => {}
                    UnaryOp::Minus => ops.push(Op::UMinus),
                }
            }
            Expr::Int(value) => match value {
                MIN_INLINE_INT..=MAX_INLINE_INT => ops.push(Op::LInt(*value as i16)),
                _ => {
                    let i = pool.len() as u16;
                    pool.push(*value);
                    ops.push(Op::LConst(i));
                }
            },
        }
    }

    let (mut ops, mut pool) = (Vec::new(), Vec::new());
    emit(expr, &mut ops, &mut pool);
    (ops, pool)
}
