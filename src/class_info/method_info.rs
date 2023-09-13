use crate::class_info::constant_pool::ConstantPool;
use crate::class_info::raw_data::RawByteBuffer;
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
pub struct MethodInfo {}

impl MethodInfo {
    pub fn from(
        _data: &mut RawByteBuffer,
        _constant_pool: &ConstantPool,
    ) -> Result<MethodInfo, Error> {
        Ok(MethodInfo {})
    }
}
