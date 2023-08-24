use std::ops::Index;
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
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
