use std::io::{Error, ErrorKind};

pub struct RawClassData {
    pub cursor: usize,
    pub data: Vec<u8>,
}

impl RawClassData {
    pub fn read_4_bytes(&mut self) -> Result<u32, Error> {
        if self.cursor + 4 >= self.data.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Can't read beyond buf boundary",
            ));
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
        if self.cursor + 2 >= self.data.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Can't read beyond buf boundary",
            ));
        }

        let offset = self.cursor;
        let buf = &self.data;

        let value = (buf[offset] as u16) << 8 | (buf[offset + 1] as u16);
        self.cursor += 2;

        Ok(value)
    }

    pub fn read_1_byte(&mut self) -> Result<u8, Error> {
        if self.cursor + 1 >= self.data.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Can't read beyond buf boundary",
            ));
        }

        let offset = self.cursor;
        let buf = &self.data;

        let value = buf[offset];
        self.cursor += 1;

        Ok(value)
    }
}
