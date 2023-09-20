use crate::class_loader::raw_data::RawByteBuffer;
use std::fmt;
use std::io::{Error, ErrorKind};

/* Some good articles related to JVM constant pool:
 * https://blogs.oracle.com/javamagazine/post/java-class-file-constant-pool
 *
 */
#[derive(Debug)]
pub struct ConstantPool {
    values: Vec<ConstantType>,
}

impl ConstantPool {
    pub fn new(data: &mut RawByteBuffer) -> Result<ConstantPool, Error> {
        let constant_pool_count = data.read_2_bytes()? as usize;

        let mut values = Vec::with_capacity(constant_pool_count);

        // constant_pool starts with 1 so we just need to push fake value as 0-based
        values.push(ConstantType::Reserved);

        for _ in 1..constant_pool_count {
            let single_value = ConstantType::from(data)?;
            values.push(single_value);
        }

        Ok(ConstantPool { values })
    }

    pub fn resolve_constant_pool_utf(&self, index: usize) -> Result<String, Error> {
        match &self.values[index] {
            ConstantType::Utf8 { value } => Ok(value.to_string()),
            ConstantType::Class { name_index } => {
                self.resolve_constant_pool_utf(*name_index as usize)
            }
            ConstantType::Fieldref {
                class_index,
                name_and_type_index,
            } => {
                let class_name = self.resolve_constant_pool_utf(*class_index as usize)?;
                let type_name = self.resolve_constant_pool_utf(*name_and_type_index as usize)?;
                Ok(format!("{}.{}", class_name, type_name))
            }
            ConstantType::Methodref {
                class_index,
                name_and_type_index,
            } => {
                let class_name = self.resolve_constant_pool_utf(*class_index as usize)?;
                let type_name = self.resolve_constant_pool_utf(*name_and_type_index as usize)?;

                Ok(format!("{}.{}", class_name, type_name))
            }

            ConstantType::String { idx } => Ok(self.resolve_constant_pool_utf(*idx as usize)?),
            ConstantType::NameAndType {
                name_index,
                descriptor_index,
            } => {
                let name = self.resolve_constant_pool_utf(*name_index as usize)?;
                let descriptor = self.resolve_constant_pool_utf(*descriptor_index as usize)?;

                Ok(format!("{}, {}", name, descriptor))
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Can't get value from constant_pool at index {}", index),
            )),
        }
    }
}

/**
 * https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4-140
 */
#[derive(Debug)]
pub enum ConstantType {
    /*
     CONSTANT_Class_info {
        u1 tag;
        u2 name_index;
    }
     */
    Class {
        name_index: u16,
    },
    /*
    CONSTANT_Fieldref_info {
        u1 tag;
        u2 class_index;
        u2 name_and_type_index;
    }
    */
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },

    /*
    CONSTANT_Methodref_info {
        u1 tag;
        u2 class_index;
        u2 name_and_type_index;
    }
     */
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },

    /*
    CONSTANT_InterfaceMethodref_info {
        u1 tag;
        u2 class_index;
        u2 name_and_type_index;
    }
    */
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },

    /*
    CONSTANT_String_info {
        u1 tag;
        u2 string_index;
    }
    */
    String {
        idx: u16,
    },
    /*
    CONSTANT_Integer_info {
        u1 tag;
        u4 bytes;
    }
    */
    Integer {
        val: u32,
    },
    /*
    CONSTANT_Float_info {
        u1 tag;
        u4 bytes;
    }
    */
    Float {
        val: u32,
    },

    /*
    CONSTANT_Long_info {
        u1 tag;
        u4 high_bytes;
        u4 low_bytes;
    }
    */
    Long {
        val: u64,
    },

    /*
    CONSTANT_Double_info {
        u1 tag;
        u4 high_bytes;
        u4 low_bytes;
    }
    */
    Double {
        val: f64,
    },
    /*
    CONSTANT_NameAndType_info {
        u1 tag;
        u2 name_index;
        u2 descriptor_index;
    }
    */
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    /*
    CONSTANT_Utf8_info {
        u1 tag;
        u2 length;
        u1 bytes[length];
    }
    */
    Utf8 {
        value: String,
    },
    MethodHandle,
    MethodType,
    InvokeDynamic,
    // Reserved type will be used to replace 0-based value in constant pool
    // which doesn't exists in real class file
    Reserved,
    Undefined,
}
impl ConstantType {
    pub fn from(data: &mut RawByteBuffer) -> Result<Self, Error> {
        let constant_tag = data.read_1_byte()?;

        match constant_tag {
            7 => Ok(Self::Class {
                name_index: data.read_2_bytes()?,
            }),
            9 => Ok(Self::Fieldref {
                class_index: data.read_2_bytes()?,
                name_and_type_index: data.read_2_bytes()?,
            }),
            10 => Ok(Self::Methodref {
                class_index: data.read_2_bytes()?,
                name_and_type_index: data.read_2_bytes()?,
            }),
            11 => Ok(Self::InterfaceMethodref {
                class_index: data.read_2_bytes()?,
                name_and_type_index: data.read_2_bytes()?,
            }),
            8 => Ok(Self::String {
                idx: data.read_2_bytes()?,
            }),
            3 => Ok(Self::Integer {
                val: data.read_4_bytes()?,
            }),
            4 => Ok(Self::Float {
                val: data.read_4_bytes()?,
            }),
            5 => Ok(Self::Long {
                val: data.read_8_bytes()?,
            }),
            6 => Ok(Self::Double {
                val: f64::from_bits(data.read_8_bytes()?),
            }),
            12 => Ok(Self::NameAndType {
                name_index: data.read_2_bytes()?,
                descriptor_index: data.read_2_bytes()?,
            }),
            1 => {
                let str_length = data.read_2_bytes()? as usize;
                Ok(Self::Utf8 {
                    value: data.read_string(str_length)?,
                })
            }
            15 => Ok(Self::MethodHandle),
            16 => Ok(Self::MethodType),
            18 => Ok(Self::InvokeDynamic),
            _ => Ok(Self::Undefined),
        }
    }
}

impl fmt::Display for ConstantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
