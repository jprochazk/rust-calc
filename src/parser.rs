use std::sync::Arc;

use crate::error::Error;
use crate::error::Result;
use crate::expr::Binary;
use crate::expr::BinaryOp;
use crate::expr::Expr;
use crate::expr::Unary;
use crate::expr::UnaryOp;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenKind;

pub struct Parser<'src> {
    lex: Lexer<'src>,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Result<Self> {
        Ok(Self {
            lex: Lexer::new(src)?,
        })
    }

    fn src(&self) -> &Arc<str> {
        self.lex.src()
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.lex.current().is(kind)
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

    fn must(&mut self, kind: TokenKind) -> Result<()> {
        if !self.at(kind) {
            return Err(Error::new(
                self.src().clone(),
                self.lex.current().span,
                format!(
                    "expected `{}` got `{}` instead",
                    kind.as_str(),
                    self.lex.current().kind.as_str()
                ),
            ));
        }
        self.bump()
    }

    fn end(&self) -> bool {
        self.at(TokenKind::Eof)
    }

    fn previous(&self) -> &Token {
        self.lex.previous()
    }

    fn current(&self) -> &Token {
        self.lex.current()
    }

    fn lexeme(&self, token: &Token) -> &str {
        &self.lex.src()[token.span]
    }
}

pub fn parse(src: &str) -> Result<Expr> {
    let mut p = Parser::new(src)?;
    let expr = parse_expr(&mut p)?;
    if !p.end() {
        return Err(Error::new(
            p.src().clone(),
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
            .map_err(|e| Error::new(p.src().clone(), token.span, e.to_string()))
            .map(Expr::Int)?;
        return Ok(value);
    }

    if p.eat(TokenKind::ParenL)? {
        let value = parse_expr(p)?;
        p.must(TokenKind::ParenR)?;
        return Ok(value);
    }

    Err(Error::new(
        p.src().clone(),
        p.current().span,
        "unexpected eof".to_string(),
    ))
}
