use crate::expr::BinaryOp;
use crate::expr::Expr;
use crate::expr::UnaryOp;

use super::op;
use super::op::Op;

pub type Bytecode = Vec<Op>;
pub type ConstPool = Vec<i64>;
pub type StackSize = usize;

pub fn compile(expr: &Expr) -> (Bytecode, ConstPool, StackSize) {
    fn emit(expr: &Expr, ops: &mut Bytecode, pool: &mut ConstPool, reg: &mut RegAlloc, dst: u8) {
        match expr {
            Expr::Binary(expr) => {
                let lhs = dst;
                emit(&expr.left, ops, pool, reg, lhs);
                let rhs = reg.alloc();
                emit(&expr.right, ops, pool, reg, rhs);
                match expr.op {
                    BinaryOp::Add => ops.push(op::BAdd(lhs, lhs, rhs)),
                    BinaryOp::Sub => ops.push(op::BSub(lhs, lhs, rhs)),
                    BinaryOp::Mul => ops.push(op::BMul(lhs, lhs, rhs)),
                    BinaryOp::Div => ops.push(op::BDiv(lhs, lhs, rhs)),
                }
                reg.free(rhs);
            }
            Expr::Unary(expr) => {
                let rhs = dst;
                emit(&expr.right, ops, pool, reg, rhs);
                match expr.op {
                    UnaryOp::Plus => {}
                    UnaryOp::Minus => ops.push(op::UMinus(rhs, rhs)),
                }
            }
            Expr::Int(value) => match value {
                MIN_INLINE_INT..=MAX_INLINE_INT => ops.push(op::LInt(dst, *value as i16)),
                _ => {
                    let i = pool.len() as u16;
                    pool.push(*value);
                    ops.push(op::LConst(dst, i));
                }
            },
        }
    }

    let (mut ops, mut pool, mut reg) = (Vec::new(), Vec::new(), RegAlloc::default());
    let dst = reg.alloc();
    emit(expr, &mut ops, &mut pool, &mut reg, dst);
    (ops, pool, reg.stack_size())
}

const MIN_INLINE_INT: i64 = i16::MIN as i64;
const MAX_INLINE_INT: i64 = i16::MAX as i64;

#[derive(Default)]
struct RegAlloc {
    current: u8,
    max: u8,
}

impl RegAlloc {
    fn alloc(&mut self) -> u8 {
        let reg = self.current;
        self.current += 1;
        self.max = std::cmp::max(self.max, self.current);
        reg
    }

    fn free(&mut self, to: u8) {
        self.current = to;
    }

    fn stack_size(&self) -> usize {
        self.max as usize
    }
}
