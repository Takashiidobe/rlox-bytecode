use crate::value::{Value, ValueArray};

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(u8),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<u32>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, byte: OpCode, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, v: Value) -> usize {
        self.constants.write(v);
        self.constants.values.len() - 1
    }
}
