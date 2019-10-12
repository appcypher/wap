use super::cursor::Cursor;
use super::limits::Limits;
use crate::traits::ReaderTrait;

pub struct TextReader {
    cursor: Cursor,
    limits: Limits,
}

impl TextReader {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::Initial,
            limits: Limits::new(),
        }
    }
}

impl ReaderTrait for TextReader {
    fn read_next(&self) -> Cursor {
        unimplemented!()
    }
}
