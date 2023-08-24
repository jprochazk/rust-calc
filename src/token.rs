use crate::span::Span;

#[derive(Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn eof(at: usize) -> Self {
        Self {
            kind: TokenKind::Eof,
            span: Span {
                start: at,
                end: at + 1,
            },
        }
    }

    pub fn is(&self, kind: TokenKind) -> bool {
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

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
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
}
