use super::cursor::Cursor;
use crate::traits::ReaderTrait;

pub struct BinaryReader {
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


impl BinaryReader {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::Initial,
        }
    }

    pub fn create_with_state(cursor: Cursor) -> Self {
        Self {
            cursor,
        }
    }
}

impl ReaderTrait for BinaryReader {
    fn read_next(&self) -> Cursor {
        return self.cursor
    }
}
