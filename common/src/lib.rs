use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Flatten;

pub type FlattenedLines = Flatten<Lines<BufReader<File>>>;

pub fn open_file(input: &str) -> io::Result<FlattenedLines> {
    let file = File::open(input)?;
    Ok(BufReader::new(file).lines().flatten())
}
