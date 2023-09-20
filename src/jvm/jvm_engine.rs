use crate::class_loader::attribute_info::AttributeInfo;
use crate::class_loader::opcodes::Opcode;
use crate::class_loader::constant_pool::ConstantPool;
use crate::class_loader::method_info::MethodInfo;
use crate::jvm::jvm_frame::JvmFrame;
use crate::jvm::jvm_type::JvmValue;

pub fn execute_bytecode(method: &MethodInfo, constant_pool: &ConstantPool) {
    if let Some(code_attribute) = method.get_code_attribute() {
        if let AttributeInfo::Code {
            name: _,
            bytecode,
            max_stack,
            max_locals,
            exception_table: _,
        } = code_attribute {
            let mut frame = JvmFrame::new(*max_stack, *max_locals);

            let mut pc = 0;

            while pc < bytecode.len() {
                let opcode = Opcode::from(bytecode[pc]).expect("Can't decode bytecode instruction");
                pc += 1;

                match opcode {
                    Opcode::Bipush => {
                        let byte_val = bytecode[pc];
                        pc += 1;

                        frame.push(JvmValue::Int(byte_val as i32));
                    }

                    Opcode::Ldc => {
                        let name_idx = bytecode[pc];
                        pc += 1;
                        let _name = constant_pool.resolve_constant_pool_utf(name_idx as usize).
                            expect(&format!("'ldc' can load constant at index {}", name_idx));
                    }

                    Opcode::Istore0 => {
                        frame.store_to_local(0);
                    }
                    Opcode::Istore1 => {
                        frame.store_to_local(1);
                    }
                    Opcode::Istore2 => {
                        frame.store_to_local(2);
                    }
                    Opcode::Istore3 => {
                        frame.store_to_local(3);
                    }

                    Opcode::Iload0 => {
                        frame.load_from_local(0);
                    }
                    Opcode::Iload1 => {
                        frame.load_from_local(1);
                    }
                    Opcode::Iload2 => {
                        frame.load_from_local(2);
                    }
                    Opcode::Iload3 => {
                        frame.load_from_local(3);
                    }

                    Opcode::Iadd => {
                        if let JvmValue::Int(first_val) = frame.pop() {
                            if let JvmValue::Int(second_val) = frame.pop() {
                                frame.push(JvmValue::Int(first_val + second_val));
                            } else {
                                panic!("Expected JvmValue::Int");
                            }
                        } else {
                            panic!("Expected JvmValue::Int");
                        }
                    }

                    Opcode::Iinc => {
                        let _index = bytecode[pc];
                        pc += 1;

                        let _value = bytecode[pc] as i8;
                        pc += 1;
                    }

                    Opcode::Ificmpeq | Opcode::Ificmpne | Opcode::Ificmplt |
                    Opcode::Ificmpge | Opcode::Ificmpgt | Opcode::Ificmple | Opcode::Goto => {
                        let _branch1 = bytecode[pc];
                        pc += 1;

                        let _branch2 = bytecode[pc];
                        pc += 1;
                    }

                    Opcode::Return => {
                        if let JvmValue::Int(val) = frame.get_local(3) {
                            println!("res = {val}");
                        } else {
                            println!("Can't find result");
                        }

                        println!("Returning from function '{}'", method.get_name());
                    }

                    Opcode::Getstatic | Opcode::Invokevirtual | Opcode::New |
                    Opcode::Invokespecial | Opcode::Invokestatic => {
                        let index = ((bytecode[pc] as usize) << 8) | (bytecode[pc + 1] as usize);
                        pc += 2;

                        let _name = constant_pool.resolve_constant_pool_utf(index);
                    }

                    _ => println!("Unprocessed opcode: {:#?}", opcode),
                }
            }
        } else {
            panic!("Can't execute bytecode b/c AttributeInfo::Code not found");
        }
    } else {
        panic!("No bytecode for 'main' function, really strange");
    }
}
