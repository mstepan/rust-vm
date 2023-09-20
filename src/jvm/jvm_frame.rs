
use crate::jvm::jvm_type::JvmValue;

pub struct JvmFrame {
    stack: Vec<JvmValue>,
}

impl JvmFrame {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: JvmValue){
        self.stack.push(value);
    }
}