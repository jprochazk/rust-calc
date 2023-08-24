#[derive(Debug)]
#[cfg_attr(feature = "random_ast", derive(arbitrary::Arbitrary))]
pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Int(#[cfg_attr(feature = "random_ast", arbitrary(with = small_i64))] i64),
}

#[cfg(feature = "random_ast")]
fn small_i64(u: &mut arbitrary::Unstructured) -> arbitrary::Result<i64> {
    u.int_in_range(i8::MIN..=i8::MAX).map(i64::from)
}

impl Expr {
    #[cfg(feature = "random_ast")]
    pub fn generate() -> Expr {
        use arbitrary::Unstructured;
        use rand::{seq::SliceRandom, thread_rng};

        let mut seed: Vec<u8> = (1..4096).map(|v| (v % u8::MAX as i32) as u8).collect();
        seed.shuffle(&mut thread_rng());
        Unstructured::new(&seed).arbitrary().unwrap()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "random_ast", derive(arbitrary::Arbitrary))]
pub struct Binary {
    pub left: Expr,
    pub op: BinaryOp,
    pub right: Expr,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[cfg(feature = "random_ast")]
impl<'a> arbitrary::Arbitrary<'a> for BinaryOp {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let options = [BinaryOp::Add, BinaryOp::Sub, BinaryOp::Mul];
        u.choose(&options).copied()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "random_ast", derive(arbitrary::Arbitrary))]
pub struct Unary {
    pub op: UnaryOp,
    pub right: Expr,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "random_ast", derive(arbitrary::Arbitrary))]
pub enum UnaryOp {
    Plus,
    Minus,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(expr) => write!(f, "({expr})"),
            Expr::Unary(expr) => write!(f, "({expr})"),
            Expr::Int(value) => write!(f, "{value}"),
        }
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { left, op, right } = self;
        write!(f, "{left} {op} {right}")
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => f.write_str("+"),
            BinaryOp::Sub => f.write_str("-"),
            BinaryOp::Mul => f.write_str("*"),
            BinaryOp::Div => f.write_str("/"),
        }
    }
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { op, right } = self;
        write!(f, "{op} {right}")
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Plus => f.write_str("+"),
            UnaryOp::Minus => f.write_str("-"),
        }
    }
}
