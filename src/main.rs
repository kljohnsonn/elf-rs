use elf::Elf64;
use elf::ParserError;
use reader::ByteReader;
use std::{
    error::Error,
    fmt::{format, Display},
    fs,
    io::Read,
    ops::Index,
    str::from_utf8,
    task::Wake,
    u32, usize,
};

mod elf;
mod reader;

fn main() -> Result<(), ParserError> {
    let data = fs::read("t").unwrap();

    let mut reader = ByteReader::new(&data);
    reader.skip(4);

    let class = reader.read_bytes(1).index(0);


    match class {
        1 => {}

        2 => {
            let elf = Elf64::parse(&data)?;
        }

        _ => {
            return Err(ParserError::Empty(format!(
                "Expected class 1 or 2, found {class}"
            )))
        }
    }

    Ok(())
}

