use super::compiler::Bytecode;
use super::compiler::ConstPool;
use super::compiler::StackSize;

pub fn eval(ops: &Bytecode, pool: &ConstPool, stack_size: StackSize) -> i64 {
    let mut stack = vec![0i64; stack_size];

    for op in ops {
        match op {
            super::op::Op::LInt(n) => stack[n.dst as usize] = n.val as i64,
            super::op::Op::LConst(n) => stack[n.dst as usize] = pool[n.idx as usize],
            super::op::Op::BAdd(n) => {
                stack[n.dst as usize] = stack[n.lhs as usize] + stack[n.rhs as usize]
            }
            super::op::Op::BSub(n) => {
                stack[n.dst as usize] = stack[n.lhs as usize] - stack[n.rhs as usize]
            }
            super::op::Op::BMul(n) => {
                stack[n.dst as usize] = stack[n.lhs as usize] * stack[n.rhs as usize]
            }
            super::op::Op::BDiv(n) => {
                stack[n.dst as usize] = stack[n.lhs as usize] / stack[n.rhs as usize]
            }
            super::op::Op::UMinus(n) => stack[n.dst as usize] = -stack[n.rhs as usize],
        }
    }

    stack[0]
}
