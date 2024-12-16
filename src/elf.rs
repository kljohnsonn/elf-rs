use std::array;
use std::str::from_utf8;
use std::task::Wake;
use std::usize;

use crate::reader::ByteReader;
use crate::Display;
use crate::Error;

pub struct Elf32 {}


#[derive(Debug)]
pub enum ProgramHeaderType {
    PtNull,
    PtLoad,
    PtDynamic,
    Undefined,
}

#[derive(Debug)]
pub enum ProgramHeaderFlag {
    Pfx,
    Pfw,
    Pfwx,
    Pfr,
    Pfrx,
    Pfrw,
    Pfrwx,
    Undefined,
}

#[derive(Debug)]
pub enum ElfFileType {
    EtNone,
    EtRel,
    EtExec,
    EtDyn,
    EtCore,
    EtOsSpec,
    EtProcSpec,
    Undefined,
}

#[derive(Debug)]
pub struct Elf64 {
    header: Elf64Header,
    program_headers: Vec<ProgramHeader64>,
    section_headers: Vec<SectionHeader64>
}

#[derive(Debug)]
pub struct SectionHeader64 {
    sh_name: u32,
    name: String,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

impl SectionHeader64 {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut reader = ByteReader::new(data);
        let sh_name = reader.read_u32()?;
        let name = String::new();
        let sh_type = reader.read_u32()?;
        let sh_flags = reader.read_u64()?;
        let sh_addr = reader.read_u64()?;
        let sh_offset = reader.read_u64()?;
        let sh_size = reader.read_u64()?;
        let sh_link = reader.read_u32()?;
        let sh_info = reader.read_u32()?;
        let sh_addralign = reader.read_u64()?;
        let sh_entsize = reader.read_u64()?;

        Ok(SectionHeader64 {
            sh_name,
            name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,

        })
    }


    pub fn read_flags(flags: u64) { todo!()} 
}

#
[derive(Debug)]
pub struct Elf64Header {
    magic: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8,
    abi: u8,
    abi_version: u8,
    padding: [u8; 7],
    e_type: ElfFileType,
    machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[derive(Debug)]
pub struct ProgramHeader64 {
    p_type: ProgramHeaderType,
    p_flag: ProgramHeaderFlag,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

impl ProgramHeader64 {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut reader = ByteReader::new(data);

        let p_type: ProgramHeaderType = ProgramHeader64::read_type(reader.read_u32()?);

        let p_flag: ProgramHeaderFlag = ProgramHeader64::read_flag(reader.read_u32()?);

        let p_offset = reader.read_u64()?;
        let p_vaddr = reader.read_u64()?;
        let p_paddr = reader.read_u64()?;

        let p_filesz = reader.read_u64()?;
        let p_memsz = reader.read_u64()?;
        let p_align = reader.read_u64()?;

        Ok(ProgramHeader64 {
            p_type,
            p_flag,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        })
    }

    pub fn read_type(data: u32) -> ProgramHeaderType {
        let p_type: ProgramHeaderType = match data {
            0 => ProgramHeaderType::PtNull,
            1 => ProgramHeaderType::PtLoad,
            2 => ProgramHeaderType::PtDynamic,
            _ => ProgramHeaderType::Undefined,
        };

        p_type
    }

    pub fn read_flag(data: u32) -> ProgramHeaderFlag {
        let p_flag: ProgramHeaderFlag = match data {
            1 => ProgramHeaderFlag::Pfx,
            2 => ProgramHeaderFlag::Pfw,
            3 => ProgramHeaderFlag::Pfwx,
            4 => ProgramHeaderFlag::Pfr,
            5 => ProgramHeaderFlag::Pfrx,
            6 => ProgramHeaderFlag::Pfrw,
            7 => ProgramHeaderFlag::Pfwx,
            _ => ProgramHeaderFlag::Undefined,
        };

        p_flag
    }
}

impl Elf64Header {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut reader = ByteReader::new(data);
        let magic: [u8; 4] = reader.read_bytes(4).try_into().unwrap();
        let class: u8 = reader.read_byte();
        let endianness = reader.read_byte();

        let version = reader.read_byte();
        let abi = reader.read_byte();
        let abi_version = reader.read_byte();

        let padding: [u8; 7] = reader.read_bytes(7).try_into().unwrap();

        let e_type = Elf64Header::read_type(reader.read_u16()?);
        let machine = reader.read_u16()?;
        let e_version = reader.read_u32()?;

        let e_entry = reader.read_u64()?;
        let e_phoff = reader.read_u64()?;
        let e_shoff = reader.read_u64()?;
        let e_flags = reader.read_u32()?;

        let e_ehsize = reader.read_u16()?;
        let e_phentsize = reader.read_u16()?;
        let e_phnum = reader.read_u16()?;

        let e_shentsize = reader.read_u16()?;
        let e_shnum = reader.read_u16()?;
        let e_shstrndx = reader.read_u16()?;

        Ok(Elf64Header {
            magic,
            class,
            endianness,
            version,
            abi,
            abi_version,
            padding,
            e_type,
            machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        })
    }

    pub fn read_type(data: u16) -> ElfFileType {
        let e_type = match data {
            0 => ElfFileType::EtNone,
            1 => ElfFileType::EtRel,
            2 => ElfFileType::EtExec,
            3 => ElfFileType::EtDyn,
            4 => ElfFileType::EtCore,
            0xFE00..=0xFEFF => ElfFileType::EtOsSpec,
            0xFF00..=0xFFFF => ElfFileType::EtProcSpec,
            _ => ElfFileType::Undefined,
        };

        e_type
    }
}

pub fn get_section_name(table: &[u8], index: usize) -> String {
    let mut name = Vec::new();
    let mut i = index;

    while table[i] != 0 {
        name.push(table[i]);
        i+=1;
    }

    String::from_utf8_lossy(&name).into_owned()


}


impl Display for ProgramHeader64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Type: {:?}", self.p_type)?;
        writeln!(f, "Flag: {:?}", self.p_flag)?;
        writeln!(f, "Offset: {}", self.p_offset)?;
        writeln!(f, "Virtual Address: {:?}", self.p_vaddr as *mut u64)?;
        writeln!(f, "Physical Address: {:?}", self.p_paddr as *mut u64)?;
        writeln!(f, "Size In Image: {}", self.p_filesz)?;
        writeln!(f, "Size In Memory: {}", self.p_memsz)?;
        writeln!(f, "Alignment (0 or 1 no alignment): {}", self.p_align)?;

        Ok(())
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum ParserError {
    Empty(String),
    Retrieval(String),
    Read(array::TryFromSliceError)
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Empty(msg) => write!(f, "Data parsing error: {}", msg),
            ParserError::Retrieval(msg) => write!(f, "Failed to retireve: {}", msg),
            ParserError::Read(msg) => write!(f, "Read failure: {}", msg),
        }
    }
}

impl Elf64 {
    pub fn parse(bytes: &[u8]) -> Result<Self, ParserError> {
        let mut reader = ByteReader::new(bytes);

        let header_data = reader.read_bytes(64);

        let header = Elf64Header::parse(header_data)?;
        let mut program_headers: Vec<ProgramHeader64> = Vec::new();
        let mut section_headers: Vec<SectionHeader64> = Vec::new();

        for _ in 0..header.e_phnum {
            let program_header_data = reader.read_bytes(header.e_phentsize.into());
            let program_header = ProgramHeader64::parse(program_header_data)?;
            println!("{}", program_header);
            program_headers.push(program_header);
        }

        let mut section_reader = ByteReader::new(&bytes[header.e_shoff as usize..]);

        for _ in 0..header.e_shnum { 
            let section_header_data = section_reader.read_bytes(header.e_shentsize as usize);
            let section_header = SectionHeader64::parse(section_header_data)?;
            println!("{}", reader.remaining().len());
            section_headers.push(section_header);
        }

        let strtab_section = &section_headers[header.e_shstrndx as usize];

        let strtab_start = strtab_section.sh_offset as usize;

        let strtab_size = strtab_section.sh_size as usize;

        let string_table = &bytes[strtab_start..strtab_start + strtab_size];

        for section in section_headers.iter_mut() {
            let section_name = get_section_name(string_table, section.sh_name as usize);
            println!("{}", section_name);
            section.name = section_name;
        }


        println!("{:?}", section_headers[0].sh_addralign);


        

        

        Ok(Elf64 {
            header,
            program_headers,
            section_headers
        })
    }
}

