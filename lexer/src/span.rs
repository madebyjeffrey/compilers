use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Span {
    pub start: usize,
    pub len: usize
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.len)
    }
}
