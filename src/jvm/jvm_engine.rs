use crate::class_loader::attribute_info::{AttributeInfo, Opcode};
use crate::class_loader::method_info::MethodInfo;
use crate::jvm::jvm_frame::JvmFrame;
use crate::jvm::jvm_type::JvmValue;

pub fn execute_bytecode(method: &MethodInfo, code_attribute: &AttributeInfo) {

    if let AttributeInfo::Code {
        name: _,
        bytecode,
        max_stack,
        max_locals,
        exception_table: _,
    } = code_attribute {
        let mut frame = JvmFrame::new(*max_stack, *max_locals);

        let mut pc = 0;
        let mut _offset = 0;

        while pc < bytecode.len() {
            let cur = &bytecode[pc];

            match cur {
                Opcode::Bipush { byte_val } => {
                    frame.push(JvmValue::Int(*byte_val as i32));
                }
                Opcode::Istore0 => {
                    frame.store_to_local(0);
                },
                Opcode::Istore1 => {
                    frame.store_to_local(1);
                },
                Opcode::Istore2 => {
                    frame.store_to_local(2);
                },
                Opcode::Istore3 => {
                    frame.store_to_local(3);
                },

                Opcode::Iload0 => {
                    frame.load_from_local(0);
                },
                Opcode::Iload1 => {
                    frame.load_from_local(1);
                },
                Opcode::Iload2 => {
                    frame.load_from_local(2);
                },
                Opcode::Iload3 => {
                    frame.load_from_local(3);
                },

                Opcode::Iadd => {
                    if let JvmValue::Int(first_val) = frame.pop() {
                        if let JvmValue::Int(second_val) = frame.pop() {
                            frame.push(JvmValue::Int(first_val + second_val));
                        }
                        else {
                            panic!("Expected JvmValue::Int");
                        }
                    }
                    else {
                        panic!("Expected JvmValue::Int");
                    }
                },

                Opcode::Return => {

                    if let JvmValue::Int(val) = frame.get_local(3) {
                        println!("res = {val}");
                    }
                    else {
                        println!("Can't find result");
                    }

                    println!("Returning from function '{}'", method.get_name());
                }
                _ => println!("Unprocessed opcode: {:#?}", cur),
            }

            // println!("{:#?}", cur);


            pc += 1;
            _offset += cur.size() as usize;
        }
    }
    else {
        panic!("Can't execute bytecode b/c AttributeInfo::Code not found");
    }
}
