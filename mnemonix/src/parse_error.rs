#[derive(Debug)]
pub struct ParseError {
    pos: usize,
    expected: &'static str,
    message: String,
}

impl ParseError {
    pub fn new(pos: usize, expected: &'static str, message: impl Into<String>) -> Self {
        ParseError {
            pos,
            expected,
            message: message.into(),
        }
    }

    /// Creates a pretty multi-line snippet showing the error in context
    pub fn pretty_error(&self, source: &str) -> String {
        let mut out = String::new();

        let line_start = source[..self.pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = source[self.pos..]
            .find('\n')
            .map(|i| self.pos + i)
            .unwrap_or(source.len());

        let line = &source[line_start..line_end];
        let col = self.pos - line_start;

        out.push_str(&format!("Error: {}\n", self.message));
        out.push_str(&format!("Expected: {}\n", self.expected));
        out.push_str(&format!("At byte {}, column {}\n\n", self.pos, col + 1));

        out.push_str(line);
        out.push('\n');
        out.push_str(&" ".repeat(col));
        out.push_str("^\n");

        out
    }
}
