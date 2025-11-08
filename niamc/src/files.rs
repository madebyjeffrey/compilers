use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use ariadne::Source;

pub fn load_file(filename: &str) -> Result<Source, io::Error> {
    let f = File::open(filename)?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(Source::from(contents.to_string()))
}
