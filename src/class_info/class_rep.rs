#![allow(dead_code)]

use std::io::{Error, ErrorKind};

use crate::class_info::raw_data::RawClassData;

/**
 * JVM file format https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
 */
#[derive(Debug)]
pub struct ClassFile {
    java_version: JavaVersion,
    constant_pool: Vec<ConstantType>,
    access_flags: Vec<ClassAccessFlag>,
    this_class: String,
    super_class: String,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
}
impl ClassFile {
    pub fn new(data: &mut RawClassData) -> Result<Self, Error> {
        let magic_number = data.read_4_bytes()?;
        assert_eq!(JAVA_MAGIC_NUMBER, magic_number);

        let java_version = Self::read_java_version(data)?;
        assert_eq!(JavaVersion::Java17, java_version);

        let constant_pool = Self::read_constant_pool(data)?;

        let access_flags = ClassAccessFlag::from_mask(data.read_2_bytes()?);

        let this_class_name = Self::read_class_name(data, &constant_pool)?;
        assert_eq!("com/max/app17/Main", this_class_name);

        let super_class_name = Self::read_class_name(data, &constant_pool)?;
        assert_eq!("java/lang/Object", super_class_name);

        let interfaces = Self::read_interfaces(data)?;

        let fields = Self::read_fields(data, &constant_pool)?;

        let _methods_count = data.read_2_bytes()?;

        Ok(Self {
            java_version,
            constant_pool,
            access_flags,
            this_class: this_class_name,
            super_class: super_class_name,
            interfaces,
            fields,
        })
    }

    fn read_java_version(data: &mut RawClassData) -> Result<JavaVersion, Error> {
        let minor_version = data.read_2_bytes()?;
        let major_version = data.read_2_bytes()?;

        Ok(JavaVersion::from(major_version, minor_version))
    }

    fn read_constant_pool(data: &mut RawClassData) -> Result<Vec<ConstantType>, Error> {
        let constant_pool_count = data.read_2_bytes()? as usize;

        let mut constant_pool = Vec::with_capacity(constant_pool_count);

        // constant_pool starts with 1 so we just need to push fake value as 0-based
        constant_pool.push(ConstantType::Undefined);

        for _ in 1..constant_pool_count {
            let constant_value = ConstantType::from(data)?;

            constant_pool.push(constant_value);
        }

        Ok(constant_pool)
    }

    fn read_class_name(
        data: &mut RawClassData,
        constant_pool: &[ConstantType],
    ) -> Result<String, Error> {
        let this_class = data.read_2_bytes()?;
        Self::resolve_constant_pool_utf(constant_pool, (this_class) as usize)
    }

    fn read_interfaces(data: &mut RawClassData) -> Result<Vec<u16>, Error> {
        let interfaces_count = data.read_2_bytes()?;

        let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_count as usize);

        for _ in 0..interfaces_count {
            let single_interface = data.read_2_bytes()?;
            interfaces.push(single_interface);
        }
        Ok(interfaces)
    }

    fn read_fields(
        data: &mut RawClassData,
        constant_pool: &[ConstantType],
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

    fn resolve_constant_pool_utf(
        constant_pool: &[ConstantType],
        index: usize,
    ) -> Result<String, Error> {
        match &constant_pool[index] {
            ConstantType::Utf8 { value } => Ok(value.to_string()),
            ConstantType::Class { name_index } => {
                Self::resolve_constant_pool_utf(constant_pool, *name_index as usize)
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Can't get value from constant_pool at index {}", index),
            )),
        }
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
    Undefined,
}
impl ConstantType {
    fn from(data: &mut RawClassData) -> Result<Self, Error> {
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

/*
https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5
field_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/
#[derive(Debug)]
struct FieldInfo {
    access_flags: Vec<FieldAcceFlag>,
    name: String,
    descriptor_name: String, // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2
    attributes_count: u16,
}

impl FieldInfo {
    fn from(data: &mut RawClassData, constant_pool: &[ConstantType]) -> Result<FieldInfo, Error> {
        // Read singe FieldInfo here
        let field_access_flags = FieldAcceFlag::from_mask(data.read_2_bytes()?);

        let name_index = data.read_2_bytes()?;
        let field_name = ClassFile::resolve_constant_pool_utf(constant_pool, name_index as usize)?;

        let descriptor_index = data.read_2_bytes()?;
        let descriptor_name =
            ClassFile::resolve_constant_pool_utf(constant_pool, descriptor_index as usize)?;

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
struct MethodInfo {}

const JAVA_MAGIC_NUMBER: u32 = 0xCA_FE_BA_BE;
