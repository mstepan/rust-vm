use crate::class_info::constant_pool::ConstantPool;
use crate::class_info::raw_data::RawByteBuffer;
use std::io::Error;

/*
https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7
*/
#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        name: String,
        bytecode: Vec<Opcode>,
        max_stack: u16,
        max_locals: u16,
        exception_table: Vec<ExceptionTableInfo>,
    },
    Other,
}

impl AttributeInfo {
    pub fn from(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<AttributeInfo, Error> {
        let attr_name_idx = data.read_2_bytes()?;
        let _attr_name = constant_pool.resolve_constant_pool_utf(attr_name_idx as usize)?;

        let attr_length = data.read_4_bytes()?;

        // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7.3
        // if attr_name == "Code" {
        //     let max_stack = data.read_2_bytes()?;
        //     let max_locals = data.read_2_bytes()?;

        //     let code_length = data.read_4_bytes()?;

        //     let mut bytecode: Vec<Opcode> = Vec::with_capacity(code_length as usize);

        //     for _ in 0..code_length {
        //         let opcode = Opcode::from(data, constant_pool)?;
        //         bytecode.push(opcode);
        //     }

        //     let exception_table_length = data.read_2_bytes()?;

        //     let mut exception_table: Vec<ExceptionTableInfo> =
        //         Vec::with_capacity(exception_table_length as usize);

        //     // read full exception table here
        //     for _ in 0..exception_table_length {
        //         exception_table.push(ExceptionTableInfo::from(data, constant_pool)?);
        //     }

        //     let code_attributes_count = data.read_2_bytes()?;
        //     // read all code attributed here if any
        //     for _ in 0..code_attributes_count {
        //         let _cur_attribute = AttributeInfo::from(data, constant_pool);
        //     }

        //     Ok(AttributeInfo::Code {
        //         name: attr_name,
        //         bytecode,
        //         max_stack,
        //         max_locals,
        //         exception_table,
        //     })
        // } else {
        //     // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7.12
        //
        //     for _ in 0..attr_length {
        //         // TODO: just skip bytes here for now
        //         data.read_1_byte();
        //     }
        //
        //     Ok(AttributeInfo::Other {})
        // }

        //todo: just consume all bytes here
        for _ in 0..attr_length {
            // TODO: just skip bytes here for now
            data.read_1_byte()?;
        }

        Ok(AttributeInfo::Other {})
    }
}

#[derive(Debug)]
pub struct ExceptionTableInfo {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTableInfo {
    pub fn from(
        data: &mut RawByteBuffer,
        _constant_pool: &ConstantPool,
    ) -> Result<ExceptionTableInfo, Error> {
        let start_pc = data.read_2_bytes()?;
        let end_pc = data.read_2_bytes()?;
        let handler_pc = data.read_2_bytes()?;
        let catch_type = data.read_2_bytes()?;
        Ok(Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}

#[derive(Debug)]
pub enum Opcode {
    Nop,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,

    Aload0,
    Aload1,
    Aload2,
    Aload3,

    Return,
    Invokespecial { name: String },

    Ldc,
    New { name: String },
    Undefined,
}
/**
 * JVM instruction set https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5
 */
impl Opcode {
    pub fn from(data: &mut RawByteBuffer, constant_pool: &ConstantPool) -> Result<Opcode, Error> {
        let code = data.read_1_byte()?;

        match code {
            0x00 => Ok(Opcode::Nop),
            0x03 => Ok(Opcode::Iconst0),
            0x04 => Ok(Opcode::Iconst1),
            0x05 => Ok(Opcode::Iconst2),
            0x06 => Ok(Opcode::Iconst3),
            0x07 => Ok(Opcode::Iconst4),
            0x08 => Ok(Opcode::Iconst5),
            0x12 => Ok(Opcode::Ldc),
            0x2A => Ok(Opcode::Aload0),
            0x2B => Ok(Opcode::Aload1),
            0x2C => Ok(Opcode::Aload2),
            0x2D => Ok(Opcode::Aload3),
            0xB1 => Ok(Opcode::Return),
            0xBB => {
                let index_byte =
                    (((data.read_1_byte()?) as usize) << 8) | data.read_1_byte()? as usize;

                let name = constant_pool.resolve_constant_pool_utf(index_byte)?;

                Ok(Opcode::New { name })
            }
            0xB7 => {
                let index_byte =
                    (((data.read_1_byte()?) as usize) << 8) | data.read_1_byte()? as usize;

                let name = constant_pool.resolve_constant_pool_utf(index_byte)?;
                Ok(Opcode::Invokespecial { name })
            }
            _ => Ok(Opcode::Undefined),
        }
    }
}
