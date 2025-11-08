use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub type SimpleFile = codespan_reporting::files::SimpleFile<String, String>;

pub fn load_file(filename: &str) -> Result<SimpleFile, io::Error> {
    let f = File::open(filename)?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(SimpleFile::new(filename.to_string(), contents.to_string()))
}
