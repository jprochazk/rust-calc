use crate::error::Error;
use crate::error::Result;
use crate::token::Token;
use crate::token::TokenKind;
use std::sync::Arc;

pub struct Lexer<'src> {
    src: Arc<str>,
    inner: logos::Lexer<'src, TokenKind>,
    previous: Token,
    current: Token,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Result<Self> {
        let mut lex = Self {
            src: src.into(),
            inner: logos::Logos::lexer(src),
            previous: Token::eof(0),
            current: Token::eof(0),
        };
        lex.bump()?;
        Ok(lex)
    }

    pub fn bump(&mut self) -> Result<()> {
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

    pub fn previous(&self) -> &Token {
        &self.previous
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn src(&self) -> &Arc<str> {
        &self.src
    }
}
