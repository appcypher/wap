use crate::Cursor;

pub trait ReaderTrait {
    fn read_next(&self) -> Cursor;
}
