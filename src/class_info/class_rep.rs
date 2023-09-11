#![allow(dead_code)]
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

const JAVA_MAGIC_NUMBER: u32 = 0xCA_FE_BA_BE;

impl ClassFile {
    pub fn new(data: &[u8]) -> Self {
        let mut cursor = 0;

        let magic_number = Self::read_4_bytes(cursor, data);
        cursor += 4;
        assert_eq!(JAVA_MAGIC_NUMBER, magic_number);

        let minor_version = Self::read_2_bytes(cursor, data);
        cursor += 2;

        let major_version = Self::read_2_bytes(cursor, data);
        cursor += 2;

        let java_version = JavaVersion::from(major_version, minor_version);

        let constant_pool_count = Self::read_2_bytes(cursor, data);
        cursor += 2;
        assert_eq!(37, constant_pool_count);

        Self {
            magic_number,
            java_version,
            constant_pool_count,
        }
    }

    fn read_4_bytes(offset: u32, data: &[u8]) -> u32 {
        let offset = offset as usize;

        (data[offset] as u32) << 24
            | (data[offset + 1] as u32) << 16
            | (data[offset + 2] as u32) << 8
            | (data[offset + 3] as u32)
    }

    fn read_2_bytes(offset: u32, data: &[u8]) -> u16 {
        let offset = offset as usize;
        (data[offset] as u16) << 8 | (data[offset + 1] as u16)
    }
}
