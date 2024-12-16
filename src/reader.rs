use crate::elf::ParserError;

#[derive(Debug)]
pub struct ByteReader<'a>(&'a [u8]);

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ByteReader(bytes)
    }

    pub fn read_bytes(&mut self, amount: usize) -> &'a [u8] {
        let (data, rest) = self.0.split_at(amount);

        self.0 = rest;

        data
    }

    pub fn read_byte(&mut self) -> u8 {
        let (data, rest) = self.0.split_at(1);

        self.0 = rest;

        data[0]
    }

    pub fn read_u64(&mut self) -> Result<u64, ParserError> {
        Ok(u64::from_le_bytes(self.read_bytes(8).try_into().unwrap()))
    }

    pub fn read_u32(&mut self) -> Result<u32, ParserError> {
        Ok(u32::from_le_bytes(self.read_bytes(4).try_into().unwrap()))
    }

    pub fn read_u16(&mut self) -> Result<u16, ParserError> {
        Ok(u16::from_le_bytes(self.read_bytes(2).try_into().unwrap()))
    }

    pub fn remaining(&mut self) -> &[u8] {
        self.0
    }

    pub fn skip(&mut self, amount: usize) {
        let (_, rest) = self.0.split_at(amount);

        self.0 = rest
    }
}

