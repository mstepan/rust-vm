#![allow(dead_code)]

use std::io::Error;

use crate::class_info::constant_pool::ConstantPool;
use crate::class_info::field_info::FieldInfo;
use crate::class_info::method_info::MethodInfo;
use crate::class_info::raw_data::RawByteBuffer;

const JAVA_MAGIC_NUMBER: u32 = 0xCA_FE_BA_BE;

/**
 * JVM file format https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
 */
#[derive(Debug)]
pub struct ClassFile {
    java_version: JavaVersion,
    constant_pool: ConstantPool,
    access_flags: Vec<ClassAccessFlag>,
    this_class: String,
    super_class: String,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
}
impl ClassFile {
    pub fn new(data: &mut RawByteBuffer) -> Result<Self, Error> {
        let magic_number = data.read_4_bytes()?;
        assert_eq!(JAVA_MAGIC_NUMBER, magic_number);

        let java_version = Self::read_java_version(data)?;
        assert_eq!(JavaVersion::Java17, java_version);

        let constant_pool = ConstantPool::new(data)?;

        let access_flags = ClassAccessFlag::from_mask(data.read_2_bytes()?);

        let this_class_name = Self::read_class_name(data, &constant_pool)?;
        assert_eq!("com/max/app17/Main", this_class_name);

        let super_class_name = Self::read_class_name(data, &constant_pool)?;
        assert_eq!("java/lang/Object", super_class_name);

        let interfaces = Self::read_interfaces(data)?;

        let fields = Self::read_fields(data, &constant_pool)?;

        let methods = Self::read_methods(data, &constant_pool)?;

        Ok(Self {
            java_version,
            constant_pool,
            access_flags,
            this_class: this_class_name,
            super_class: super_class_name,
            interfaces,
            fields,
            methods,
        })
    }

    fn read_java_version(data: &mut RawByteBuffer) -> Result<JavaVersion, Error> {
        let minor_version = data.read_2_bytes()?;
        let major_version = data.read_2_bytes()?;

        Ok(JavaVersion::from(major_version, minor_version))
    }

    fn read_class_name(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<String, Error> {
        let this_class = data.read_2_bytes()?;
        constant_pool.resolve_constant_pool_utf((this_class) as usize)
    }

    fn read_interfaces(data: &mut RawByteBuffer) -> Result<Vec<u16>, Error> {
        let interfaces_count = data.read_2_bytes()?;

        let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_count as usize);

        for _ in 0..interfaces_count {
            let single_interface = data.read_2_bytes()?;
            interfaces.push(single_interface);
        }
        Ok(interfaces)
    }

    fn read_fields(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<Vec<FieldInfo>, Error> {
        let fields_count = data.read_2_bytes()?;

        let mut fields: Vec<FieldInfo> = vec![];

        for _ in 0..fields_count {
            // Read singe FieldInfo here
            let field_info = FieldInfo::from(data, constant_pool)?;
            fields.push(field_info);
        }

        Ok(fields)
    }

    fn read_methods(
        data: &mut RawByteBuffer,
        constant_pool: &ConstantPool,
    ) -> Result<Vec<MethodInfo>, Error> {
        let methods_count = data.read_2_bytes()?;

        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            let single_method = MethodInfo::from(data, constant_pool)?;
            methods.push(single_method);
        }

        Ok(methods)
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Copy, Clone)]
enum ClassAccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}
impl ClassAccessFlag {
    fn values() -> Vec<ClassAccessFlag> {
        vec![
            ClassAccessFlag::Public,
            ClassAccessFlag::Final,
            ClassAccessFlag::Super,
            ClassAccessFlag::Interface,
            ClassAccessFlag::Abstract,
            ClassAccessFlag::Synthetic,
            ClassAccessFlag::Annotation,
            ClassAccessFlag::Enum,
        ]
    }

    fn from_mask(flags_mask: u16) -> Vec<ClassAccessFlag> {
        let mut access_flags: Vec<ClassAccessFlag> = Vec::new();

        for single_flag in ClassAccessFlag::values() {
            if flags_mask & (single_flag as u16) != 0 {
                access_flags.push(single_flag);
            }
        }

        access_flags
    }
}
