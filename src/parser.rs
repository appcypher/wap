use super::traits::ReaderTrait;
use crate::reader::State;


pub struct Parser<R> {
    pub(crate) reader: R,
}

impl<R: ReaderTrait> Parser<R> {
    pub fn read_next(&self) -> State {
        self.reader.read_next()
    }
}
