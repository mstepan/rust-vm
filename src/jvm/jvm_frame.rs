use crate::jvm::jvm_type::JvmValue;

pub struct JvmFrame {
    stack: Vec<JvmValue>,
    local_slots: Vec<JvmValue>,
}

impl JvmFrame {
    pub fn new(stack_length: u16, locals_length: u16) -> Self {
        Self {
            stack: Vec::with_capacity(stack_length as usize),
            local_slots: vec![JvmValue::Undefined; locals_length as usize],
        }
    }

    pub fn push(&mut self, value: JvmValue) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> JvmValue{
        self.stack.pop().expect("Can't be empty here")
    }

    pub fn store_to_local(&mut self, local_idx: u16) {
        self.local_slots[local_idx as usize] = self.stack.pop().expect("Can't be empty here");
    }

    pub fn load_from_local(&mut self, local_idx: u16) {
        self.stack.push( self.local_slots[local_idx as usize]);
    }
}