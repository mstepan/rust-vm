use crate::class_loader::attribute_info::{AttributeInfo, Opcode};
use crate::class_loader::constant_pool::ConstantPool;
use crate::class_loader::raw_data::RawByteBuffer;
use std::io::Error;
/*
https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.6

method_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/
#[derive(Debug)]
pub struct MethodInfo {
    access_flags: Vec<MethodAccessFlag>,
    name: String,
    descriptor: String,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn from(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<MethodInfo, Error> {
        let access_flags = MethodAccessFlag::from_mask(data.read_2_bytes()?);

        let name = Self::read_name_or_descriptor(data, constant_pool)?;
        let descriptor = Self::read_name_or_descriptor(data, constant_pool)?;

        let attributes_count = data.read_2_bytes()?;

        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            // Read single attribute here
            let single_attribute = AttributeInfo::from(data, constant_pool)?;
            attributes.push(single_attribute);
        }

        Ok(MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn read_name_or_descriptor(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<String, Error> {
        let name_index = data.read_2_bytes()?;
        let name = constant_pool.resolve_constant_pool_utf(name_index as usize)?;
        Ok(name)
    }

    pub fn is_main(&self) -> bool {
        self.name == "main"
    }

    pub fn get_code_attribute(&self) -> Option<&AttributeInfo> {
        for single_attribute in &self.attributes {

            if let AttributeInfo::Code {
                name,
                bytecode: _,
                max_stack: _,
                max_locals: _,
                exception_table: _
            } = single_attribute {
                return Some(single_attribute);
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthehic = 0x1000,
}
impl MethodAccessFlag {
    fn values() -> Vec<MethodAccessFlag> {
        vec![
            MethodAccessFlag::Public,
            MethodAccessFlag::Private,
            MethodAccessFlag::Protected,
            MethodAccessFlag::Static,
            MethodAccessFlag::Final,
            MethodAccessFlag::Synchronized,
            MethodAccessFlag::Bridge,
            MethodAccessFlag::Varargs,
            MethodAccessFlag::Native,
            MethodAccessFlag::Abstract,
            MethodAccessFlag::Strict,
            MethodAccessFlag::Synthehic,
        ]
    }

    fn from_mask(flags_mask: u16) -> Vec<MethodAccessFlag> {
        let mut access_flags: Vec<MethodAccessFlag> = Vec::new();

        for single_flag in MethodAccessFlag::values() {
            if flags_mask & (single_flag as u16) != 0 {
                access_flags.push(single_flag);
            }
        }

        access_flags
    }
}
