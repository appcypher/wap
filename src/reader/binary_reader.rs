use super::cursor::Cursor;
use super::limits::Limits;
use crate::traits::ReaderTrait;

pub struct BinaryReader {
    cursor: Cursor,
    limits: Limits,
}

impl BinaryReader {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::Initial,
            limits: Limits::new(),
        }
    }
}

impl ReaderTrait for BinaryReader {
    fn read_next(&self) -> Cursor {
        return self.cursor
    }
}
