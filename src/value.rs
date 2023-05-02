pub type Value = f64;

#[derive(Default, Debug, Clone)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray::default()
    }

    pub fn write(&mut self, v: Value) {
        self.values.push(v);
    }
}
