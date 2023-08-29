use super::compiler::Bytecode;
use super::compiler::ConstPool;
use super::op::Op;

pub fn eval(ops: &Bytecode, pool: &ConstPool) -> i64 {
    let mut stack = Vec::with_capacity(128);

    for op in ops {
        match op {
            Op::LInt(value) => stack.push(*value as i64),
            Op::LConst(index) => stack.push(unsafe { *pool.get_unchecked(*index as usize) }),
            Op::BAdd => {
                let right = unsafe { stack.pop().unwrap_unchecked() };
                let left = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(left + right);
            }
            Op::BSub => {
                let right = unsafe { stack.pop().unwrap_unchecked() };
                let left = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(left - right);
            }
            Op::BMul => {
                let right = unsafe { stack.pop().unwrap_unchecked() };
                let left = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(left * right);
            }
            Op::BDiv => {
                let right = unsafe { stack.pop().unwrap_unchecked() };
                let left = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(left / right);
            }
            Op::UMinus => {
                let right = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(-right);
            }
        }
    }

    unsafe { stack.pop().unwrap_unchecked() }
}
