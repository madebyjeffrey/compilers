use common::mapped_file::MappedFile;
use common::span::Span;
use regex::Regex;

pub struct Preprocessor {
    pub comment_start: Regex,
    pub comment_end: Regex,
}

impl Preprocessor {
    pub fn new(comment_start: &str, commend_end: &str) -> Preprocessor {
        Preprocessor {
            comment_start: Regex::new(comment_start).unwrap(),
            comment_end: Regex::new(commend_end).unwrap(),
        }
    }

    pub fn process(&self, source: &MappedFile) -> String {
        let mut removals = Vec::<Span>::new();
        let mut start = 0;

        loop {
            if let Some(m) = self.comment_start.find_at(&source.contents, start) {
                let r = m.range();

                if let Some(m2) = self.comment_end.find_at(&source.contents, m.end()) {
                    let r2 = m2.range();

                    removals.push(Span::combine_ranges(r, r2));
                    start = m.end();
                    continue;
                }
            }

            break;
        }

        let sliced_and_diced: String = removals
            .iter()
            .map(|&span| &source.contents[span.start..][..span.len])
            .collect();

        sliced_and_diced
    }
}

/// Maps an offset in the slimmed string back to the original string,
/// given a list of removed ranges in the original string.
///
/// `removed` must be sorted and non-overlapping.
/// Returns `None` if the offset is past the end of the slimmed string.
#[allow(dead_code)]
fn map_slimmed_to_original_chars(offset: usize, removed: &[Span]) -> Option<usize> {
    let mut shift = 0;

    for span in removed {
        let start = span.start;
        let len = span.len;

        // The slimmed offset lies entirely before this removed range.
        if offset < start - shift {
            return Some(offset + shift);
        }

        // The slimmed offset lies after this removed range.
        shift += len;
    }

    // If we’ve passed all removed ranges, apply total shift.
    Some(offset + shift)
}

/// Maps an offset in the original string to the slimmed string,
/// given a list of removed ranges in the original string.
///
/// `removed` must be sorted and non-overlapping.
/// Returns `None` if the offset lies inside a removed range.
#[allow(dead_code)]
fn map_original_to_slimmed_chars(offset: usize, removed: &[Span]) -> Option<usize> {
    let mut shift = 0;

    for &span in removed {
        let start = span.start;
        let len = span.len;
        let end = start + len;

        if offset < start {
            // Offset is before this removed range — apply previous shifts.
            return Some(offset - shift);
        } else if offset < end {
            // Offset lies inside a removed range.
            return None;
        } else {
            // Offset is after this removed range — accumulate shift.
            shift += end - start;
        }
    }

    // After all ranges — apply total shift.
    Some(offset - shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ascii_mapping() {
        // Original: abcdefghi
        // Removed: [2..4) => "cd", [6..7) => "g"
        let removed = vec![Span::new(2, 2), Span::new(6, 1)];

        // Slimmed: abefhi (6 chars)
        assert_eq!(map_slimmed_to_original_chars(0, &removed), Some(0));
        assert_eq!(map_slimmed_to_original_chars(1, &removed), Some(1));
        assert_eq!(map_slimmed_to_original_chars(2, &removed), Some(4));
        assert_eq!(map_slimmed_to_original_chars(3, &removed), Some(5));
        assert_eq!(map_slimmed_to_original_chars(4, &removed), Some(7));
        assert_eq!(map_slimmed_to_original_chars(5, &removed), Some(8));
        assert_eq!(map_slimmed_to_original_chars(6, &removed), Some(9));

        // Reverse direction (original → slimmed)
        assert_eq!(map_original_to_slimmed_chars(0, &removed), Some(0));
        assert_eq!(map_original_to_slimmed_chars(1, &removed), Some(1));
        assert_eq!(map_original_to_slimmed_chars(2, &removed), None); // inside "cd"
        assert_eq!(map_original_to_slimmed_chars(3, &removed), None);
        assert_eq!(map_original_to_slimmed_chars(4, &removed), Some(2));
        assert_eq!(map_original_to_slimmed_chars(5, &removed), Some(3));
        assert_eq!(map_original_to_slimmed_chars(6, &removed), None); // inside "g"
        assert_eq!(map_original_to_slimmed_chars(7, &removed), Some(4));
        assert_eq!(map_original_to_slimmed_chars(8, &removed), Some(5));
        assert_eq!(map_original_to_slimmed_chars(9, &removed), Some(6));
    }

    #[test]
    fn utf8_multibyte_characters() {
        // Original: aβcδεfgh
        // Removed (by char index): [1..3) => "βc", [4..5) => "ε"
        let removed = vec![Span::new(1, 2), Span::new(4, 1)];

        // Slimmed: aδfgh
        assert_eq!(map_slimmed_to_original_chars(0, &removed), Some(0)); // a
        assert_eq!(map_slimmed_to_original_chars(1, &removed), Some(3)); // δ
        assert_eq!(map_slimmed_to_original_chars(2, &removed), Some(5)); // f
        assert_eq!(map_slimmed_to_original_chars(3, &removed), Some(6)); // g
        assert_eq!(map_slimmed_to_original_chars(4, &removed), Some(7)); // h
        assert_eq!(map_slimmed_to_original_chars(5, &removed), Some(8)); // end

        assert_eq!(map_original_to_slimmed_chars(0, &removed), Some(0)); // a
        assert_eq!(map_original_to_slimmed_chars(1, &removed), None); // β
        assert_eq!(map_original_to_slimmed_chars(2, &removed), None); // c
        assert_eq!(map_original_to_slimmed_chars(3, &removed), Some(1)); // δ
        assert_eq!(map_original_to_slimmed_chars(4, &removed), None); // ε
        assert_eq!(map_original_to_slimmed_chars(5, &removed), Some(2)); // f
        assert_eq!(map_original_to_slimmed_chars(6, &removed), Some(3)); // g
        assert_eq!(map_original_to_slimmed_chars(7, &removed), Some(4)); // h
        assert_eq!(map_original_to_slimmed_chars(8, &removed), Some(5)); // end
    }

    #[test]
    fn empty_removals() {
        let removed: Vec<Span> = vec![];

        for i in 0..5 {
            assert_eq!(map_slimmed_to_original_chars(i, &removed), Some(i));
            assert_eq!(map_original_to_slimmed_chars(i, &removed), Some(i));
        }
    }

    #[test]
    fn removal_at_start() {
        // Removed first 3 characters
        let removed = vec![Span::new(0, 3)];

        assert_eq!(map_slimmed_to_original_chars(0, &removed), Some(3));
        assert_eq!(map_slimmed_to_original_chars(5, &removed), Some(8));

        assert_eq!(map_original_to_slimmed_chars(0, &removed), None);
        assert_eq!(map_original_to_slimmed_chars(3, &removed), Some(0));
    }

    #[test]
    fn removal_at_end() {
        // Removed last 2 characters
        let removed = vec![Span::new(5, 2)];

        assert_eq!(map_slimmed_to_original_chars(0, &removed), Some(0));
        assert_eq!(map_slimmed_to_original_chars(5, &removed), Some(7));

        assert_eq!(map_original_to_slimmed_chars(5, &removed), None);
        assert_eq!(map_original_to_slimmed_chars(4, &removed), Some(4));
    }

    #[test]
    fn offset_past_end_returns_some() {
        // Ensure mapping past end just accumulates shift
        let removed = vec![Span::new(2, 2)];
        assert_eq!(map_slimmed_to_original_chars(10, &removed), Some(12));
    }
}
