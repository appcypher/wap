use super::cursor::Cursor;
use crate::traits::ReaderTrait;

pub struct TextReader {
    cursor: Cursor,
}

// GLOBAL LIMIT: // held by Reader
//    type_index_limit, mem_index_limit, table_index_limit, global_index_limit,
//    func_index_limit,
//    func_stack_type_match (call)
//    section_order
// LOCAL LIMIT: // held by FunctionBodyReader
//    local_index_limit, block_index_limit,
//    stack_type_match (ops, return, block entry, block return)


impl TextReader {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::Initial,
        }
    }
}

impl ReaderTrait for TextReader {
    fn read_next(&self) -> Cursor {
        unimplemented!()
    }
}
