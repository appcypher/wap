use wap::{wasm, wat};
use wap::Cursor;

fn main() {
    // Create new wasm binary Parser
    let parser = wasm::Parser::new();

    // Move cursor
    match parser.read_next() {
        Cursor::Initial => {
            println!("{:?}", Cursor::Initial);
        },
    }
}
