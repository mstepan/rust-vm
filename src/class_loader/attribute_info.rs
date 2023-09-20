use std::io::Error;

use crate::class_loader::constant_pool::ConstantPool;
use crate::class_loader::raw_data::RawByteBuffer;

/*
https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7
*/
#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        name: String,
        bytecode: Vec<u8>,
        max_stack: u16,
        max_locals: u16,
        exception_table: Vec<ExceptionTableInfo>,
    },
    NotParsedYet,
}

impl AttributeInfo {
    /*
    Attributes.
    https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
    */
    pub fn from(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<AttributeInfo, Error> {
        let attr_name = constant_pool.resolve_constant_pool_utf(data.read_2_bytes()? as usize)?;
        let attr_length = data.read_4_bytes()?;

        /*
        4.7.3. The Code Attribute
        https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
         */
        if attr_name == "Code" {
            let max_stack = data.read_2_bytes()?;
            let max_locals = data.read_2_bytes()?;

            let bytecode_length = data.read_4_bytes()?;

            let mut bytecode = Vec::with_capacity(bytecode_length as usize);

            // Fully read all opcodes representing function 'code' body
            for _ in 0..bytecode_length {
                bytecode.push(data.read_1_byte()?);
            }

            let exception_table_length = data.read_2_bytes()?;

            let mut exception_table: Vec<ExceptionTableInfo> =
                Vec::with_capacity(exception_table_length as usize);

            // read full exception table here
            for _ in 0..exception_table_length {
                exception_table.push(ExceptionTableInfo::from(data, constant_pool)?);
            }

            let code_attributes_count = data.read_2_bytes()?;
            // read all code attributed here if any
            for _ in 0..code_attributes_count {
                let _cur_attribute = AttributeInfo::from(data, constant_pool);
            }

            Ok(AttributeInfo::Code {
                name: attr_name,
                bytecode,
                max_stack,
                max_locals,
                exception_table,
            })
        } else {
            // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.12

            for _ in 0..attr_length {
                // TODO: just skip bytes here for now
                data.read_1_byte()?;
            }

            Ok(AttributeInfo::NotParsedYet {})
        }
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

