use crate::debug::disassemble_chunk;
use chunk::{Chunk, OpCode};
use std::time::Instant;
use vm::Vm;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let now = Instant::now();
    let mut vm = Vm::default();
    let mut c = Chunk::new();

    let constant = c.add_constant(1.2);
    c.write(OpCode::Constant(constant as _), 123);

    let constant = c.add_constant(3.4);
    c.write(OpCode::Constant(constant as _), 123);

    c.write(OpCode::Add, 123);

    let constant = c.add_constant(5.6);
    c.write(OpCode::Constant(constant as _), 123);

    c.write(OpCode::Divide, 123);
    c.write(OpCode::Negate, 123);
    c.write(OpCode::Return, 123);

    disassemble_chunk(&c, "test chunk");

    vm.interpret(c);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
