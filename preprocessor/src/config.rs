use regex::Regex;

pub struct PreprocessorConfig {
    pub comment_start: Regex,
    pub commend_end: Regex
}