use std::sync::Arc;

use crate::span::Span;

#[derive(Debug)]
pub struct Error {
    src: Arc<str>,
    span: Span,
    message: String,
}

impl Error {
    pub fn new(src: Arc<str>, span: Span, message: String) -> Self {
        Error { src, span, message }
    }

    pub fn report(&self) -> String {
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

pub type Result<T, E = Error> = std::result::Result<T, E>;
