use crate::class_loader::constant_pool::ConstantPool;
use crate::class_loader::raw_data::RawByteBuffer;
use std::io::{Error, ErrorKind};

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
    NotParsedYet,
}

impl AttributeInfo {
    /*
    Attributes.
    https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7.3
    */
    pub fn from(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<AttributeInfo, Error> {
        let attr_name = constant_pool.resolve_constant_pool_utf(data.read_2_bytes()? as usize)?;
        let attr_length = data.read_4_bytes()?;

        /*
        4.7.3. The Code Attribute
        https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7.3
         */
        if attr_name == "Code" {
            let max_stack = data.read_2_bytes()?;
            let max_locals = data.read_2_bytes()?;

            let mut code_length = data.read_4_bytes()?;

            let mut bytecode: Vec<Opcode> = Vec::with_capacity(code_length as usize);

            while code_length != 0 {
                let opcode = Opcode::from(data, constant_pool)?;

                code_length -= opcode.size() as u32;
                bytecode.push(opcode);
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
            // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7.12

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

#[derive(Debug)]
pub enum Opcode {
    Nop,

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.iconst_i
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.bipush
    Bipush { byte_val: u8 },

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.aload_n
    Aload0,
    Aload1,
    Aload2,
    Aload3,

    // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.istore_n
    Istore0,
    Istore1,
    Istore2,
    Istore3,

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.iadd
    Iadd,

    Return,

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.getstatic
    Getstatic { name: String },

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.invokevirtual
    Invokevirtual { name: String },

    Invokespecial { name: String },

    // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.ldc
    Ldc { name: String },

    //https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.iload_n
    Iload0,
    Iload1,
    Iload2,
    Iload3,

    New { name: String },
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

            0x10 => {
                let byte_val = data.read_1_byte()?;
                Ok(Opcode::Bipush { byte_val })
            }
            0x12 => {
                let name_idx = data.read_1_byte()?;
                let name = constant_pool.resolve_constant_pool_utf(name_idx as usize)?;
                Ok(Opcode::Ldc { name })
            }
            0x1A => Ok(Opcode::Iload0),
            0x1B => Ok(Opcode::Iload1),
            0x1C => Ok(Opcode::Iload2),
            0x1D => Ok(Opcode::Iload3),

            0x2A => Ok(Opcode::Aload0),
            0x2B => Ok(Opcode::Aload1),
            0x2C => Ok(Opcode::Aload2),
            0x2D => Ok(Opcode::Aload3),

            0x3B => Ok(Opcode::Istore0),
            0x3C => Ok(Opcode::Istore1),
            0x3D => Ok(Opcode::Istore2),
            0x3E => Ok(Opcode::Istore3),

            0x60 => Ok(Opcode::Iadd),
            0xB1 => Ok(Opcode::Return),

            0xB2 => {
                let name = Self::read_index_byte_and_lokup_name(data, constant_pool)?;
                Ok(Opcode::Getstatic { name })
            }

            0xB6 => {
                let name = Self::read_index_byte_and_lokup_name(data, constant_pool)?;
                Ok(Opcode::Invokevirtual { name })
            }

            0xBB => {
                let name = Self::read_index_byte_and_lokup_name(data, constant_pool)?;
                Ok(Opcode::New { name })
            }
            0xB7 => {
                let name = Self::read_index_byte_and_lokup_name(data, constant_pool)?;
                Ok(Opcode::Invokespecial { name })
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Can't recognize opcode value {}", code),
            )),
        }
    }

    fn read_index_byte_and_lokup_name(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<String, Error> {
        let index_byte = (((data.read_1_byte()?) as usize) << 8) | data.read_1_byte()? as usize;
        constant_pool.resolve_constant_pool_utf(index_byte)
    }

    pub fn size(&self) -> u8 {
        match &self {
            Opcode::New { name: _ } => 3,
            Opcode::Invokespecial { name: _ } => 3,
            Opcode::Getstatic { name: _ } => 3,
            Opcode::Invokevirtual { name: _ } => 3,
            Opcode::Ldc { name: _ } => 2,
            Opcode::Bipush { byte_val: _ } => 2,
            _ => 1,
        }
    }
}
