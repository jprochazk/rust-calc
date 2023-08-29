use super::compiler::Bytecode;
use super::compiler::ConstPool;
use super::compiler::StackSize;
use super::op::Op;

pub fn eval(ops: &Bytecode, pool: &ConstPool, stack_size: StackSize) -> i64 {
    let mut stack = Stack::new(stack_size);

    for op in ops {
        match op {
            Op::LInt(value) => stack.push(*value as i64),
            Op::LConst(index) => stack.push(unsafe { *pool.get_unchecked(*index as usize) }),
            Op::BAdd => {
                let right = stack.pop();
                let left = stack.pop();
                stack.push(left + right);
            }
            Op::BSub => {
                let right = stack.pop();
                let left = stack.pop();
                stack.push(left - right);
            }
            Op::BMul => {
                let right = stack.pop();
                let left = stack.pop();
                stack.push(left * right);
            }
            Op::BDiv => {
                let right = stack.pop();
                let left = stack.pop();
                stack.push(left / right);
            }
            Op::UMinus => {
                let right = stack.pop();
                stack.push(-right);
            }
        }
    }

    stack.pop()
}

struct Stack {
    buffer: Vec<i64>,
    ptr: usize,
}

impl Stack {
    fn new(size: StackSize) -> Self {
        Self {
            buffer: vec![0i64; size],
            ptr: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, value: i64) {
        unsafe {
            let slot = self.buffer.get_unchecked_mut(self.ptr);
            self.ptr += 1;
            *slot = value;
        }
    }

    #[inline(always)]
    fn pop(&mut self) -> i64 {
        unsafe {
            let slot = self.buffer.get_unchecked(self.ptr);
            self.ptr -= 1;
            *slot
        }
    }
}
