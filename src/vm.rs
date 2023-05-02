use crate::chunk::OpCode;
use crate::debug::disassemble_instruction;
use crate::{chunk::Chunk, value::Value};

#[derive(Default, Debug, Clone)]
pub struct Vm {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl Vm {
    pub fn interpret(&mut self, chunk: Chunk) {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) {
        loop {
            if self.ip >= self.stack.len() {
                break;
            }
            let op = &self.chunk.code[self.ip];

            match op {
                OpCode::Constant(n) => {
                    let constant = &self.chunk.constants.values[*n as usize];
                    println!("{}", n);
                    self.stack.push(*constant);
                }
                OpCode::Add => {
                    self.binary_op(|x, y| x + y);
                }
                OpCode::Subtract => {
                    self.binary_op(|x, y| x - y);
                }
                OpCode::Multiply => {
                    self.binary_op(|x, y| x * y);
                }
                OpCode::Divide => {
                    self.binary_op(|x, y| x / y);
                }
                OpCode::Negate => {
                    let negated = -self.pop();
                    self.stack.push(negated);
                }
                OpCode::Return => {
                    println!("{}", &self.pop());
                }
            }
            self.ip += 1;
        }
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty stack")
    }

    fn binary_op(&mut self, f: fn(f64, f64) -> f64) {
        let b = self.pop();
        let a = self.pop();
        self.stack.push(f(a, b))
    }

    pub fn debug_trace_execution(&self) {
        print!("          ");
        for slot in &self.stack {
            println!("[{}]", &slot);
        }
        println!();
        disassemble_instruction(&self.chunk, self.ip);
    }
}
