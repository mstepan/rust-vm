use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Opcode {
    Nop,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.iconst_i
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.bipush
    Bipush,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.aload_n
    Aload0,
    Aload1,
    Aload2,
    Aload3,

    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.istore_n
    Istore0,
    Istore1,
    Istore2,
    Istore3,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.iadd
    Iadd,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.iinc
    Iinc,

    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.if_icmp_cond
    Ificmpeq,
    Ificmpne,
    Ificmplt,
    Ificmpge,
    Ificmpgt,
    Ificmple,

    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.goto
    Goto,

    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.ireturn
    Ireturn,
    Return,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.getstatic
    Getstatic,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.invokevirtual
    Invokevirtual,

    Invokespecial,

    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.return
    Invokestatic,

    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.ldc
    Ldc,

    //https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5.iload_n
    Iload0,
    Iload1,
    Iload2,
    Iload3,

    New,
}

/**
 * JVM instruction set https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-6.html#jvms-6.5
 */
impl Opcode {
    pub fn from(code: u8) -> Result<Opcode, Error> {
        match code {
            0x00 => Ok(Opcode::Nop),

            0x03 => Ok(Opcode::Iconst0),
            0x04 => Ok(Opcode::Iconst1),
            0x05 => Ok(Opcode::Iconst2),
            0x06 => Ok(Opcode::Iconst3),
            0x07 => Ok(Opcode::Iconst4),
            0x08 => Ok(Opcode::Iconst5),

            0x10 => Ok(Opcode::Bipush),
            0x12 => Ok(Opcode::Ldc),
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

            0x84 => Ok(Opcode::Iinc),

            0x9F => Ok(Opcode::Ificmpeq),
            0xA0 => Ok(Opcode::Ificmpne),
            0xA1 => Ok(Opcode::Ificmplt),
            0xA2 => Ok(Opcode::Ificmpge),
            0xA3 => Ok(Opcode::Ificmpgt),
            0xA4 => Ok(Opcode::Ificmple),

            0xA7 => Ok(Opcode::Goto),

            0xAC => Ok(Opcode::Ireturn),
            0xB1 => Ok(Opcode::Return),

            0xB2 => Ok(Opcode::Getstatic),
            0xB6 => Ok(Opcode::Invokevirtual),
            0xBB => Ok(Opcode::New),
            0xB7 => Ok(Opcode::Invokespecial),
            0xB8 => Ok(Opcode::Invokestatic),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Can't recognize opcode value: '{}'", code),
            )),
        }
    }
}
