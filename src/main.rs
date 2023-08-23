use std::ops::Index;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug)]
pub struct Error {
    src: Arc<str>,
    span: Span,
    message: String,
}

impl Error {
    fn new(src: Arc<str>, span: Span, message: String) -> Self {
        Error { src, span, message }
    }

    fn report(&self) -> String {
        use core::fmt::Write;

        let src = &self.src;
        let span = &self.span;
        let message = &self.message;

        // empty span
        if span.start == span.end {
            return message.clone();
        }

        let mut out = String::new();
        writeln!(&mut out, "{message}:").unwrap();
        let line_start = src[..span.start].rfind('\n').map(|v| v + 1).unwrap_or(0);
        let line_end = src[span.start..]
            .find('\n')
            .map(|v| v + span.start)
            .unwrap_or(src.len());
        writeln!(&mut out, "  {}", &src[line_start..line_end]).unwrap();
        let cursor_pos = span.start - line_start;
        let cursor_len = if span.end > line_end {
            line_end - span.start
        } else {
            span.end - span.start
        };
        writeln!(
            &mut out,
            "  {:w$}{:^<l$}",
            "",
            "^",
            w = cursor_pos,
            l = cursor_len
        )
        .unwrap();

        out
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.message, self.span)
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Span {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<Span> for Range<usize> {
    #[inline]
    fn from(value: Span) -> Self {
        Range {
            start: value.start,
            end: value.end,
        }
    }
}

impl Index<Span> for str {
    type Output = <str as Index<Range<usize>>>::Output;

    #[inline]
    fn index(&self, index: Span) -> &Self::Output {
        self.index(Range::from(index))
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Clone, Copy)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    fn eof(at: usize) -> Self {
        Self {
            kind: TokenKind::Eof,
            span: Span {
                start: at,
                end: at + 1,
            },
        }
    }

    fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}

#[derive(Clone, Copy, logos::Logos, PartialEq, Eq)]
#[logos(skip r"[\s\n]+")]
pub enum TokenKind {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("(")]
    ParenL,
    #[token(")")]
    ParenR,
    #[regex(r"\d+")]
    Int,

    Eof,
}

/* impl TokenKind {
    fn as_str(&self) -> &'static str {
        use TokenKind::*;
        match self {
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            ParenL => "(",
            ParenR => ")",
            Int => "int",
            Eof => "eof",
        }
    }
} */

pub struct Lexer<'src> {
    src: Arc<str>,
    inner: logos::Lexer<'src, TokenKind>,
    previous: Token,
    current: Token,
}

impl<'src> Lexer<'src> {
    fn new(src: &'src str) -> Result<Self> {
        let mut lex = Self {
            src: src.into(),
            inner: logos::Logos::lexer(src),
            previous: Token::eof(0),
            current: Token::eof(0),
        };
        lex.bump()?;
        Ok(lex)
    }

    fn bump(&mut self) -> Result<()> {
        std::mem::swap(&mut self.previous, &mut self.current);
        let token = self.inner.next();
        let span = self.inner.span().into();
        self.current = match token {
            Some(Ok(kind)) => Token::new(kind, span),
            None => Token::eof(self.previous.span.end),
            Some(Err(())) => {
                return Err(Error::new(
                    self.src.clone(),
                    span,
                    format!("unexpected token `{}`", &self.src[span]),
                ))
            }
        };
        Ok(())
    }
}

pub struct Parser<'src> {
    lex: Lexer<'src>,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Result<Self> {
        Ok(Self {
            lex: Lexer::new(src)?,
        })
    }

    fn src(&self) -> Arc<str> {
        self.lex.src.clone()
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.lex.current.is(kind)
    }

    fn bump(&mut self) -> Result<()> {
        self.lex.bump()
    }

    fn eat(&mut self, kind: TokenKind) -> Result<bool> {
        if self.at(kind) {
            self.bump()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /* fn must(&mut self, kind: TokenKind) -> Result<()> {
        if !self.at(kind) {
            return Err(Error::new(
                self.src(),
                self.lex.current.span,
                format!(
                    "expected `{}` got `{}` instead",
                    kind.as_str(),
                    self.lex.current.kind.as_str()
                ),
            ));
        }
        self.bump()
    } */

    fn end(&self) -> bool {
        self.at(TokenKind::Eof)
    }

    fn previous(&self) -> &Token {
        &self.lex.previous
    }

    fn current(&self) -> &Token {
        &self.lex.current
    }

    fn lexeme(&self, token: &Token) -> &str {
        &self.lex.src[token.span]
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Int(i64),
}

#[derive(Debug)]
pub struct Binary {
    left: Expr,
    op: BinaryOp,
    right: Expr,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Unary {
    op: UnaryOp,
    right: Expr,
}

#[derive(Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
}

fn parse(src: &str) -> Result<Expr> {
    let mut p = Parser::new(src)?;
    let expr = parse_expr(&mut p)?;
    if !p.end() {
        return Err(Error::new(
            p.src(),
            p.current().span,
            format!("unexpected token `{}`", p.lexeme(p.current())),
        ));
    }
    Ok(expr)
}

fn parse_expr(p: &mut Parser) -> Result<Expr> {
    parse_mul_or_div(p)
}

fn parse_mul_or_div(p: &mut Parser) -> Result<Expr> {
    let mut left = parse_add_or_sub(p)?;

    while !p.end() {
        let op = match p.current().kind {
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            _ => break,
        };
        p.bump()?; // bump op
        let right = parse_add_or_sub(p)?;

        left = Expr::Binary(Box::new(Binary { left, op, right }));
    }

    Ok(left)
}

fn parse_add_or_sub(p: &mut Parser) -> Result<Expr> {
    let mut left = parse_unary(p)?;

    while !p.end() {
        let op = match p.current().kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            _ => break,
        };
        p.bump()?; // bump op
        let right = parse_unary(p)?;

        left = Expr::Binary(Box::new(Binary { left, op, right }));
    }

    Ok(left)
}

fn parse_unary(p: &mut Parser) -> Result<Expr> {
    let op = match p.current().kind {
        TokenKind::Plus => UnaryOp::Plus,
        TokenKind::Minus => UnaryOp::Minus,
        _ => return parse_primary(p),
    };
    p.bump()?;
    let right = parse_unary(p)?;

    Ok(Expr::Unary(Box::new(Unary { op, right })))
}

fn parse_primary(p: &mut Parser) -> Result<Expr> {
    if p.eat(TokenKind::Int)? {
        let token = p.previous();
        let value = p
            .lexeme(token)
            .parse::<i64>()
            .map_err(|e| Error::new(p.src(), token.span, e.to_string()))
            .map(Expr::Int)?;
        return Ok(value);
    }

    if p.eat(TokenKind::ParenL)? {
        let mut open = 1;
        while p.eat(TokenKind::ParenL)? {
            open += 1;
        }

        let value = parse_expr(p)?;

        let mut closed = 0;
        while p.eat(TokenKind::ParenR)? {
            closed += 1;
        }

        if closed != open {
            return Err(Error::new(
                p.src(),
                p.previous().span,
                format!("expected {open} closing parentheses, got {closed}"),
            ));
        }

        return Ok(value);
    }

    Err(Error::new(
        p.src(),
        p.current().span,
        "unexpected eof".to_string(),
    ))
}

fn fold(expr: &Expr) -> i64 {
    match expr {
        Expr::Binary(expr) => {
            let left = fold(&expr.left);
            let right = fold(&expr.right);
            match expr.op {
                BinaryOp::Add => left + right,
                BinaryOp::Sub => left - right,
                BinaryOp::Mul => left * right,
                BinaryOp::Div => left / right,
            }
        }
        Expr::Unary(expr) => {
            let right = fold(&expr.right);
            match expr.op {
                UnaryOp::Plus => right,
                UnaryOp::Minus => -right,
            }
        }
        Expr::Int(value) => *value,
    }
}

enum Op {
    LInt(i64),
    BAdd,
    BSub,
    BMul,
    BDiv,
    UMinus,
}

fn compile(expr: &Expr) -> Vec<Op> {
    fn emit(expr: &Expr, ops: &mut Vec<Op>) {
        match expr {
            Expr::Binary(expr) => {
                emit(&expr.left, ops);
                emit(&expr.right, ops);
                let op = match expr.op {
                    BinaryOp::Add => Op::BAdd,
                    BinaryOp::Sub => Op::BSub,
                    BinaryOp::Mul => Op::BMul,
                    BinaryOp::Div => Op::BDiv,
                };
                ops.push(op);
            }
            Expr::Unary(expr) => {
                emit(&expr.right, ops);
                let op = match expr.op {
                    UnaryOp::Plus => return,
                    UnaryOp::Minus => Op::UMinus,
                };
                ops.push(op);
            }
            Expr::Int(value) => ops.push(Op::LInt(*value)),
        }
    }

    let mut ops = Vec::new();
    emit(expr, &mut ops);
    ops
}

fn exec(ops: &[Op]) -> i64 {
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

fn main() {
    use rustyline::{error::ReadlineError, DefaultEditor};

    let mut ed = DefaultEditor::new().unwrap();
    loop {
        match ed.readline("> ") {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let expr = match parse(&line) {
                    Ok(expr) => expr,
                    Err(e) => {
                        eprintln!("\n{}", e.report());
                        continue;
                    }
                };
                let ops = compile(&expr);
                let value = exec(&ops);
                println!("{value}");
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}
