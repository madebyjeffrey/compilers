use std::fmt;
use std::fmt::Display;
use std::ops::Range;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Span {
    pub start: usize,
    pub len: usize
}

impl Span {
    pub fn new(start: usize, len: usize) -> Span {
        Span { start, len }
    }

    pub fn combine_ranges(r1: Range<usize>, r2: Range<usize>) -> Span {
        if r1.start < r2.start {
            Span::new(r1.start, r2.end - r1.start)
        } else {
            Span::new(r2.start, r1.end- r2.start)
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.len)
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

        assert_eq!(span.start, 3);
        assert_eq!(span.len, 5);
    }
}