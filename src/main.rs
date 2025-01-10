use clap::Parser;
use elf::check_class;
use elf::Elf64;
use elf::ElfType;
use elf::ParserError;
use reader::ByteReader;
use std::{
    error::Error,
    fmt::Display,
    fs,    
    ops::Index,
 
};

mod elf;
mod cli;
mod reader;
mod display;


fn main() {
    let cli = cli::ElfDump::parse();


    if let Some(header) = cli.header {
        if fs::exists(&header).expect("Can't check existence of file") {
            let file = fs::read(&header).unwrap();
            let mut reader = ByteReader::new(&file);

            match check_class(&mut reader) {
                ElfType::Elf32 => {
                }

                ElfType::Elf64 => {
                    let elf = Elf64::parse(&file).unwrap();

                    println!("{}", elf.header);   
                }

                ElfType::Invalid => {
                    println!("Elf file may be corrupted")
                }
            }
        } else {
            println!("File does not exist")
        }
    }
}




fn temp() -> Result<(), ParserError> {
    let data = fs::read("t").unwrap();

    let mut reader = ByteReader::new(&data);
    reader.skip(4);

    let class = reader.read_bytes(1).index(0);


    match class {
        1 => {}

        2 => {
            let elf = Elf64::parse(&data)?;
            println!("{:?}", elf.section_headers)
        }

        _ => {
            return Err(ParserError::Empty(format!(
                "Expected class 1 or 2, found {class}"
            )))
        }
    } 
    Ok(())

}

