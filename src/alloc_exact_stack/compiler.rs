use crate::expr::BinaryOp;
use crate::expr::Expr;
use crate::expr::UnaryOp;

use super::op::Op;

pub type Bytecode = Vec<Op>;
pub type ConstPool = Vec<i64>;
pub type StackSize = usize;

const MIN_INLINE_INT: i64 = i16::MIN as i64;
const MAX_INLINE_INT: i64 = i16::MAX as i64;

#[derive(Default)]
struct StackAlloc {
    current: StackSize,
    max: StackSize,
}

impl StackAlloc {
    fn push(&mut self) {
        self.current += 1;
        self.max = std::cmp::max(self.max, self.current);
    }

    fn pop(&mut self) {
        self.current -= 1;
    }

    fn finish(self) -> StackSize {
        self.max
    }
}

pub fn compile(expr: &Expr) -> (Bytecode, ConstPool, StackSize) {
    fn emit(expr: &Expr, ops: &mut Bytecode, pool: &mut ConstPool, stack: &mut StackAlloc) {
        match expr {
            Expr::Binary(expr) => {
                emit(&expr.left, ops, pool, stack);
                emit(&expr.right, ops, pool, stack);
                stack.pop();
                match expr.op {
                    BinaryOp::Add => ops.push(Op::BAdd),
                    BinaryOp::Sub => ops.push(Op::BSub),
                    BinaryOp::Mul => ops.push(Op::BMul),
                    BinaryOp::Div => ops.push(Op::BDiv),
                }
            }
            Expr::Unary(expr) => {
                emit(&expr.right, ops, pool, stack);
                match expr.op {
                    UnaryOp::Plus => {}
                    UnaryOp::Minus => ops.push(Op::UMinus),
                }
            }
            Expr::Int(value) => {
                match value {
                    MIN_INLINE_INT..=MAX_INLINE_INT => ops.push(Op::LInt(*value as i16)),
                    _ => {
                        let i = pool.len() as u16;
                        pool.push(*value);
                        ops.push(Op::LConst(i));
                    }
                }
                stack.push();
            }
        }
    }

    let (mut ops, mut pool, mut stack) = (Vec::new(), Vec::new(), StackAlloc::default());
    emit(expr, &mut ops, &mut pool, &mut stack);
    (ops, pool, stack.finish())
}
