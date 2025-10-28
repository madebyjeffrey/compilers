use regex::Regex;
use common::mapped_file::MappedFile;
use common::span::Span;

pub struct Preprocessor {
    pub comment_start: Regex,
    pub comment_end: Regex
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
                    let r2 = m.range();

                    removals.push(Span::new(
                }
            }
        }
    }
}
