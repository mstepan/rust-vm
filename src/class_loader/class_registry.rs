use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

use crate::class_loader::class_file::ClassFile;
use crate::class_loader::raw_data::RawByteBuffer;

pub struct ClassRegistry {}

impl ClassRegistry {
    pub fn load_class(&self, class_name: &str) -> Result<ClassFile, Error> {
        let full_class_path = format!("java/{}.class", Self::class_name_to_path(class_name));

        println!(
            "Loading class into JVM class name: '{}', path: '{}'",
            class_name, full_class_path
        );

        let mut class_file_buf =
            BufReader::new(File::open(full_class_path).expect("Can't open file"));
        let mut buf = Vec::new();

        class_file_buf.read_to_end(&mut buf)?;

        let mut raw_file_data = RawByteBuffer {
            cursor: 0,
            data: buf,
        };

        let loaded_class = ClassFile::new(&mut raw_file_data)?;

        Ok(loaded_class)
    }

    fn class_name_to_path(class_name: &str) -> String {
        class_name.replace('.', "/")
    }
}
