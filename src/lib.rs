mod parser;
mod reader;

pub mod error;
pub mod traits;

pub use reader::Cursor;

pub mod wasm {
    use crate::parser::Parser as P;
    use crate::reader::BinaryReader;
    pub type Parser = P<BinaryReader>;

    impl Parser {
        pub fn new() -> Self {
            Self {
                reader: BinaryReader::new(),
            }
        }
    }
}

pub mod wat {
    use crate::parser::Parser as P;
    use crate::reader::TextReader;
    pub type Parser = P<TextReader>;

    impl Parser {
        pub fn new() -> Self {
            Self {
                reader: TextReader::new(),
            }
        }
    }
}
