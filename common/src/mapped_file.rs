use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Range;
use codespan_reporting::files::{Error, Files};

#[derive()]
pub struct SourceFile {
    pub id: Option<String>,
    pub filename: String,
    pub contents: String,
    lines_offsets: Vec<usize>,
    pub length: usize,
}

impl SourceFile {
    #[allow(unused)]
    pub fn from_file(id: Option<String>, filename: &str) -> Result<Self, String> {
        let f = File::open(filename);

        match f {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                let result = buf_reader.read_to_string(&mut contents);

                match result {
                    Ok(size) => {
                        let offsets = Self::offsets(&contents);

                        Ok(Self {
                            id,
                            filename: filename.to_string(),
                            contents,
                            length: size,
                            lines_offsets: offsets,
                        })
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn from_string(id: Option<String>, input: &str) -> Self {
        let offsets = Self::offsets(&input);

        SourceFile {
            id,
            filename: "(buffer)".parse().unwrap(),
            contents: input.to_string(),
            lines_offsets: offsets,
            length: input.len(),
        }
    }

    pub fn offsets(input: &str) -> Vec<usize> {
        let mut offsets = Vec::new();

        for (offset, c) in input.char_indices() {
            if c == '\n' {
                offsets.push(offset);
            }
        }

        offsets
    }
    
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn line_pos_from_offset(&self, offset: usize) -> Option<(usize, usize)> {
        if self.length == 0 {
            return None;
        }

        if self.lines_offsets.len() > 0 {
            // handle the first line
            if offset <= self.lines_offsets[0] {
                return Some((1, offset + 1));
            }

            for i in 1..self.lines_offsets.len() {
                if self.lines_offsets[i] >= offset {
                    return Some((i + 1, offset - self.lines_offsets[i - 1]));
                }
            }
        }

        if offset < self.length {
            return Some((
                self.lines_offsets.len() + 1,
                match self.lines_offsets.last() {
                    Some(last) => offset - last,
                    None => offset + 1,
                },
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST1: &str = "12345\n12345\n12345\n";
    static TEST2: &str = "12345\n12345\n12345\n123";

    static TEST3: &str = "";

    static TEST4: &str = "12345";

    #[test]
    fn file_map_should_be_created() {
        let fm = SourceFile::from_string(None, TEST1);

        assert_eq!(fm.lines_offsets.len(), 3);

        let fm2 = SourceFile::from_string(None, TEST2);

        // last line is handled separately now
        assert_eq!(fm2.lines_offsets.len(), 3);
    }

    #[test]
    fn file_map_should_get_line_position_at_start() {
        let fm = SourceFile::from_string(None, TEST1);

        let case1 = fm.line_pos_from_offset(0);
        assert_eq!(case1.unwrap(), (1, 1));
    }

    #[test]
    fn file_map_should_get_first_line_positions() {
        let fm = SourceFile::from_string(None, TEST1);

        assert_eq!(fm.line_pos_from_offset(0).unwrap(), (1, 1));
        assert_eq!(fm.line_pos_from_offset(1).unwrap(), (1, 2));
        assert_eq!(fm.line_pos_from_offset(2).unwrap(), (1, 3));
        assert_eq!(fm.line_pos_from_offset(3).unwrap(), (1, 4));
        assert_eq!(fm.line_pos_from_offset(4).unwrap(), (1, 5));
        assert_eq!(fm.line_pos_from_offset(5).unwrap(), (1, 6));
    }

    #[test]
    fn file_map_should_get_second_line_positions() {
        let fm = SourceFile::from_string(None, TEST1);

        assert_eq!(fm.line_pos_from_offset(6).unwrap(), (2, 1));
        assert_eq!(fm.line_pos_from_offset(7).unwrap(), (2, 2));
        assert_eq!(fm.line_pos_from_offset(8).unwrap(), (2, 3));
        assert_eq!(fm.line_pos_from_offset(9).unwrap(), (2, 4));
        assert_eq!(fm.line_pos_from_offset(10).unwrap(), (2, 5));
        assert_eq!(fm.line_pos_from_offset(11).unwrap(), (2, 6));
    }

    #[test]
    fn file_map_should_get_last_line_positions_without_ending_lf() {
        let fm = SourceFile::from_string(None, TEST2);

        assert_eq!(fm.line_pos_from_offset(18).unwrap(), (4, 1));
        assert_eq!(fm.line_pos_from_offset(19).unwrap(), (4, 2));
        assert_eq!(fm.line_pos_from_offset(20).unwrap(), (4, 3));
        assert_eq!(fm.line_pos_from_offset(21), None);
    }

    #[test]
    fn file_map_should_work_empty() {
        let fm = SourceFile::from_string(None, TEST3);

        assert_eq!(fm.line_pos_from_offset(0), None);
        assert_eq!(fm.line_pos_from_offset(5), None);
    }

    #[test]
    fn file_map_should_work_with_one_line_no_lf() {
        let fm = SourceFile::from_string(None, TEST4);

        assert_eq!(fm.line_pos_from_offset(0).unwrap(), (1, 1));
        assert_eq!(fm.line_pos_from_offset(5), None);
    }
}
