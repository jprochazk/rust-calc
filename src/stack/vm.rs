use super::compiler::Bytecode;
use super::compiler::ConstPool;
use super::op::Op;

pub fn eval(ops: &Bytecode, pool: &ConstPool) -> i64 {
    let mut stack = Vec::with_capacity(128);

    for op in ops {
        match op {
            Op::LInt(value) => stack.push(*value as i64),
            Op::LConst(index) => stack.push(pool[*index as usize]),
            Op::BAdd => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            }
            Op::BSub => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
            }
            Op::BMul => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            Op::BDiv => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left / right);
            }
            Op::UMinus => {
                let right = stack.pop().unwrap();
                stack.push(-right);
            }
        }
    }

    stack.pop().unwrap()
}
