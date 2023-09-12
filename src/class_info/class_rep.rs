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
enum ConstantType {
    Class,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    InvokeDynamic,
    Undefined,
}
impl ConstantType {
    fn from(tag: u8) -> Self {
        match tag {
            7 => Self::Class,
            9 => Self::Fieldref,
            10 => Self::Methodref,
            11 => Self::InterfaceMethodref,
            8 => Self::String,
            3 => Self::Integer,
            4 => Self::Float,
            5 => Self::Long,
            6 => Self::Double,
            12 => Self::NameAndType,
            1 => Self::Utf8,
            15 => Self::MethodHandle,
            16 => Self::MethodType,
            18 => Self::InvokeDynamic,
            _ => Self::Undefined,
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

        for constant_idx in 1..constant_pool_count {
            let constant_tag = data.read_1_byte()?;
            let constant_type = ConstantType::from(constant_tag);
        }

        Ok(Self {
            magic_number,
            java_version,
            constant_pool_count,
        })
    }
}
