use crate::class_loader::constant_pool::ConstantPool;
use crate::class_loader::raw_data::RawByteBuffer;
use std::io::Error;
/*
https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.5
field_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/
#[derive(Debug)]
pub struct FieldInfo {
    access_flags: Vec<FieldAcceFlag>,
    name: String,
    descriptor_name: String, // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.3.2
    attributes_count: u16,
}

impl FieldInfo {
    pub fn from(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<FieldInfo, Error> {
        // Read singe FieldInfo here
        let field_access_flags = FieldAcceFlag::from_mask(data.read_2_bytes()?);

        let name_index = data.read_2_bytes()?;
        let field_name = constant_pool.resolve_constant_pool_utf(name_index as usize)?;

        let descriptor_index = data.read_2_bytes()?;
        let descriptor_name = constant_pool.resolve_constant_pool_utf(descriptor_index as usize)?;

        let attributes_count = data.read_2_bytes()?;

        // Read all attribute_info[attributes_count] here
        for _ in 0..attributes_count {
            todo!("read single attribute_info here")
        }

        Ok(FieldInfo {
            access_flags: field_access_flags,
            name: field_name,
            descriptor_name,
            attributes_count,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum FieldAcceFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthehic = 0x1000,
    Enum = 0x4000,
}

impl FieldAcceFlag {
    fn values() -> Vec<FieldAcceFlag> {
        vec![
            FieldAcceFlag::Public,
            FieldAcceFlag::Private,
            FieldAcceFlag::Protected,
            FieldAcceFlag::Static,
            FieldAcceFlag::Final,
            FieldAcceFlag::Volatile,
            FieldAcceFlag::Transient,
            FieldAcceFlag::Synthehic,
            FieldAcceFlag::Enum,
        ]
    }

    fn from_mask(flags_mask: u16) -> Vec<FieldAcceFlag> {
        let mut access_flags: Vec<FieldAcceFlag> = Vec::new();

        for single_flag in FieldAcceFlag::values() {
            if flags_mask & (single_flag as u16) != 0 {
                access_flags.push(single_flag);
            }
        }

        access_flags
    }
}
