#![allow(dead_code)]

use std::io::Error;

use crate::class_info::raw_data::RawClassData;

/**
 * JVM file format https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
 */
#[derive(Debug)]
pub struct ClassFile {
    magic_number: u32,
    java_version: JavaVersion,
    constant_pool_count: u16,
    constant_info: Vec<ConstantType>,
}

#[derive(Debug)]
enum JavaVersion {
    Java18,
    Java17,
    Java16,
    Java15,
    Java14,
    Java13,
    Java12,
    Java11,
    Java10,
    Java9,
    Java8,
    Undefined,
}

impl JavaVersion {
    /**
     * https://mkyong.com/java/list-of-java-class-file-major-version-numbers/
     */
    pub fn from(major: u16, minor: u16) -> JavaVersion {
        match (major, minor) {
            (0x3E, 0) => JavaVersion::Java18,
            (0x3D, 0) => JavaVersion::Java17,
            (0x3C, 0) => JavaVersion::Java16,
            (0x3B, 0) => JavaVersion::Java15,
            (0x3A, 0) => JavaVersion::Java14,
            (0x39, 0) => JavaVersion::Java13,
            (0x38, 0) => JavaVersion::Java12,
            (0x37, 0) => JavaVersion::Java11,
            (0x36, 0) => JavaVersion::Java10,
            (0x35, 0) => JavaVersion::Java9,
            (0x34, 0) => JavaVersion::Java8,
            _ => JavaVersion::Undefined,
        }
    }
}

/**
 * https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4-140
 */
#[derive(Debug)]
enum ConstantType {
    /*
     CONSTANT_Class_info {
        u1 tag;
        u2 name_index;
    }
     */
    Class {
        name_index: u16,
    },
    Fieldref,

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
    InterfaceMethodref,
    String,
    Integer,
    Float,
    Long,
    Double,
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
    Undefined,
}
impl ConstantType {
    fn from(data: &mut RawClassData) -> Result<Self, Error> {
        let constant_tag = data.read_1_byte()?;

        match constant_tag {
            7 => Ok(Self::Class {
                name_index: data.read_2_bytes()?,
            }),
            9 => Ok(Self::Fieldref),
            10 => Ok(Self::Methodref {
                class_index: data.read_2_bytes()?,
                name_and_type_index: data.read_2_bytes()?,
            }),
            11 => Ok(Self::InterfaceMethodref),
            8 => Ok(Self::String),
            3 => Ok(Self::Integer),
            4 => Ok(Self::Float),
            5 => Ok(Self::Long),
            6 => Ok(Self::Double),
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

const JAVA_MAGIC_NUMBER: u32 = 0xCA_FE_BA_BE;

impl ClassFile {
    pub fn new(data: &mut RawClassData) -> Result<Self, Error> {
        let magic_number = data.read_4_bytes()?;
        assert_eq!(JAVA_MAGIC_NUMBER, magic_number);

        let minor_version = data.read_2_bytes()?;

        let major_version = data.read_2_bytes()?;

        let java_version = JavaVersion::from(major_version, minor_version);

        let constant_pool_count = data.read_2_bytes()?;
        assert_eq!(37, constant_pool_count);

        let mut constant_info = Vec::with_capacity((constant_pool_count - 1) as usize);

        for _constant_idx in 1..constant_pool_count {
            let constant_value = ConstantType::from(data)?;

            constant_info.push(constant_value);
        }

        Ok(Self {
            magic_number,
            java_version,
            constant_pool_count,
            constant_info,
        })
    }
}
