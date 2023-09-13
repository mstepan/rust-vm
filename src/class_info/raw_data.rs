use std::io::{Error, ErrorKind};

use std::str;

pub struct RawClassData {
    pub cursor: usize,
    pub data: Vec<u8>,
}

impl RawClassData {
    pub fn read_8_bytes(&mut self) -> Result<u64, Error> {
        if let Some(error) = self.check_boundary(8) {
            return Err(error);
        }

        let offset = self.cursor;
        let buf = &self.data;

        let value = (buf[offset] as u64) << 56
            | (buf[offset] as u64) << 48
            | (buf[offset] as u64) << 40
            | (buf[offset] as u64) << 32
            | (buf[offset] as u64) << 24
            | (buf[offset + 1] as u64) << 16
            | (buf[offset + 2] as u64) << 8
            | (buf[offset + 3] as u64);

        self.cursor += 8;
        Ok(value)
    }

    pub fn read_4_bytes(&mut self) -> Result<u32, Error> {
        if let Some(error) = self.check_boundary(4) {
            return Err(error);
        }

        let offset = self.cursor;
        let buf = &self.data;

        let value = (buf[offset] as u32) << 24
            | (buf[offset + 1] as u32) << 16
            | (buf[offset + 2] as u32) << 8
            | (buf[offset + 3] as u32);

        self.cursor += 4;
        Ok(value)
    }

    pub fn read_2_bytes(&mut self) -> Result<u16, Error> {
        if let Some(error) = self.check_boundary(2) {
            return Err(error);
        }

        let offset = self.cursor;
        let buf = &self.data;

        let value = (buf[offset] as u16) << 8 | (buf[offset + 1] as u16);
        self.cursor += 2;

        Ok(value)
    }

    pub fn read_1_byte(&mut self) -> Result<u8, Error> {
        if let Some(error) = self.check_boundary(1) {
            return Err(error);
        }

        let offset = self.cursor;
        let buf = &self.data;

        let value = buf[offset];
        self.cursor += 1;

        Ok(value)
    }

    pub fn read_string(&mut self, length: usize) -> Result<String, Error> {
        if let Some(error) = self.check_boundary(length) {
            return Err(error);
        }

        let res = str::from_utf8(&self.data[self.cursor..self.cursor + length]).map_err(|err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Can't parse UTF-8 String {}", err),
            )
        })?;

        self.cursor += length;

        Ok(res.to_string())
    }

    fn check_boundary(&self, length: usize) -> Option<Error> {
        if self.cursor + length >= self.data.len() {
            return Some(Error::new(
                ErrorKind::InvalidData,
                "Can't read beyond buf boundary",
            ));
        }

        None
    }
}
