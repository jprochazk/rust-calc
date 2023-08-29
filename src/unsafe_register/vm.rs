use super::compiler::Bytecode;
use super::compiler::ConstPool;
use super::compiler::StackSize;

macro_rules! set {
    ($a:ident, $i:expr, $v:expr) => {
        unsafe { *$a.get_unchecked_mut($i as usize) = $v }
    };
}

macro_rules! get {
    ($a:ident, $i:expr) => {{
        #[allow(unused_unsafe)]
        let v = unsafe { *$a.get_unchecked($i as usize) };
        v
    }};
}

pub fn eval(ops: &Bytecode, pool: &ConstPool, stack_size: StackSize) -> i64 {
    let mut stack = vec![0i64; stack_size];

    for op in ops {
        match op {
            super::op::Op::LInt(n) => set!(stack, n.dst, n.val as i64),
            super::op::Op::LConst(n) => set!(stack, n.dst, get!(pool, n.idx as usize)),
            super::op::Op::BAdd(n) => {
                set!(stack, n.dst, get!(stack, n.lhs) + get!(stack, n.rhs))
            }
            super::op::Op::BSub(n) => {
                set!(stack, n.dst, get!(stack, n.lhs) - get!(stack, n.rhs))
            }
            super::op::Op::BMul(n) => {
                set!(stack, n.dst, get!(stack, n.lhs) * get!(stack, n.rhs))
            }
            super::op::Op::BDiv(n) => {
                set!(stack, n.dst, get!(stack, n.lhs) / get!(stack, n.rhs))
            }
            super::op::Op::UMinus(n) => {
                set!(stack, n.dst, -get!(stack, n.rhs))
            }
        }
    }

    get!(stack, 0)
}
