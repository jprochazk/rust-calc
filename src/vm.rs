use crate::op::Op;

pub fn eval(ops: &[Op]) -> i64 {
    let mut stack = Vec::with_capacity(16);

    for op in ops {
        match op {
            Op::LInt(value) => stack.push(*value),
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
