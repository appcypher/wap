use super::traits::ReaderTrait;
use crate::reader::Cursor;


pub struct Parser<R> {
    pub(crate) reader: R,
}

impl<R: ReaderTrait> Parser<R> {
    pub fn read_next(&self) -> Cursor {
        self.reader.read_next()
    }
}
