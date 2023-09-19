use crate::class_loader::attribute_info::Opcode;

pub fn execute_bytecode(bytecode: &[Opcode]) {

    println!("{:#?}", bytecode);

}
