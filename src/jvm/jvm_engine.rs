use crate::class_loader::attribute_info::Opcode;
use crate::jvm::jvm_type::JvmValue;
use crate::jvm::jvm_frame::JvmFrame;

pub fn execute_bytecode(bytecode: &[Opcode]) {

    let mut pc = 0;
    let mut _offset = 0;

    let mut frame = JvmFrame::new();

    while pc < bytecode.len() {
        let cur = &bytecode[pc];

        match cur {
            Opcode::Bipush {byte_val} => {
                frame.push(JvmValue::Int(*byte_val as i32));
            },
            _ => println!("{:#?}", cur),
        }

        // println!("{:#?}", cur);


        pc += 1;
        _offset += cur.size() as usize;
    }

    // println!("{:#?}", bytecode);
}
