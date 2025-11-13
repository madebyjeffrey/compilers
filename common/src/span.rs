use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::ops::Range;
use ariadne::Span as ASpan;
use crate::source_file::Id;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Span {
    _unit: Id,
    _start: usize,
    _len: usize
}

impl Span {
    pub fn new(start: usize, len: usize) -> Span {
        Span { _unit: Id::Main, _start: start, _len: len }
    }

    pub fn new_with_unit(start: usize, len: usize, unit: Id) -> Span {
        Span { _unit: unit, _start: start, _len: len }
    }

    pub fn combine_ranges(r1: Range<usize>, r2: Range<usize>) -> Span {
        if r1.start < r2.start {
            Span::new(r1.start, r2.end - r1.start)
        } else {
            Span::new(r2.start, r1.end- r2.start)
        }
    }

    pub fn expand(&mut self, len: usize) {
        self._len += len;
    }

    pub fn range(&self) -> Range<usize> {
        self._start..self._start + self._len
    }
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Span {
        Span::new(r.start, r.end - r.start)
    }
}

impl ASpan for Span {
    type SourceId = Id;

    fn source(&self) -> &Self::SourceId {
        &self._unit
    }

    fn start(&self) -> usize {
        self._start
    }

    fn end(&self) -> usize {
        self._start.saturating_add(self._len)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}..{}]", self._start, self.end())
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare the 'value' field to determine the order
        self._start.partial_cmp(&other._start)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    // want to test for the ranges being correct especially for length
    #[test]
    fn test_combine() {
        let s = "0123456789";
        let r1 = Regex::new("3").unwrap();
        let r2 = Regex::new("7").unwrap();

        let range1 = r1.find(s).unwrap().range();

        assert_eq!(range1.start, 3);
        assert_eq!(range1.end, 4);

        let range2 = r2.find(s).unwrap().range();

        assert_eq!(range2.start, 7);
        assert_eq!(range2.end, 8);

        let span = Span::combine_ranges(range1, range2);

        assert_eq!(span._start, 3);
        assert_eq!(span._len, 5);
    }

    #[test]
    fn test_range() {
        let span1 = Span::new(0, 2);

        assert_eq!(span1.range(), 0..2);
    }
}