use std::io::Cursor;

mod parser;

#[derive(Clone, Copy)]
pub struct PMXFile {}

impl PMXFile {
    pub fn new() -> Result<Self, ()> {
        Ok(Self {  })
    }

    pub fn from_cursor(cur: Cursor<u8>) -> Result<Self, ()> {
        parser::PMXParser::from_bytes(cur)
            .parse();

        Ok(Self {  })
    }
}
