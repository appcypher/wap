## Event-driven WebAssembly Binary and Text Parser

### FEATURES

- [ ] Event-driven
- [ ] WebAssembly binary parsing
- [ ] WebAssembly text parsing
- [ ] Partial parsing
- [ ] Text format dump

### THE WASM SPEC
- https://webassembly.github.io/spec/
- https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md


### WELL-FORMED UTF-8 BYTE SEQUENCES
Based on Unicode Standard 11.0, Section 3.9, Table 3-7.

| Code Points        | First Byte   | Second Byte    | Third Byte    | Fourth Byte   |
|:-------------------|:-------------|:---------------|:--------------|:--------------|
| U+0000..U+007F     | 00..7F       |                |               |               |
| U+0080..U+07FF     | C2..DF       | 80..BF         |               |               |
| U+0800..U+0FFF     | E0           | A0..BF         | 80..BF        |               |
| U+1000..U+CFFF     | E1..EC       | 80..BF         | 80..BF        |               |
| U+D000..U+D7FF     | ED           | 80..9F         | 80..BF        |               |
| U+E000..U+FFFF     | EE..EF       | 80..BF         | 80..BF        |               |
| U+10000..U+3FFFF   | F0           | 90..BF         | 80..BF        | 80..BF        |
| U+40000..U+FFFFF   | F1..F3       | 80..BF         | 80..BF        | 80..BF        |
| U+100000..U+10FFFF | F4           | 80..8F         | 80..BF        | 80..BF        |

### LEB128
TODO


### VALIDATION

- Out of bounds block index
- Out of bounds type index
- Out of bounds function index
- Out of bounds local index
- Out of bounds memory index
- Out of bounds table index
- Out of bounds global index
- Block input type and input value match (types and arity)
- Block result type and result value match (types and arity)
- Valid hexadecimal float
- Correctness of operand types in opcode
- Valid respective LEB values
- Valid utf-8 strings


