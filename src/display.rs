use std::fmt::{write, Display};

use crate::elf::{self, Elf64Header};


impl Display for Elf64Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Elf Header:")?;
        write!(f, " Magic:")?;
        writeln!(f, "               {:?}", self.magic)?;
        write!(f, " Class:")?;
        match &self.class {
            2 => writeln!(f, "              ELF32")?,

            _=> writeln!(f, "               UNKNOWN")?
        }


        Ok(())
    }
}



