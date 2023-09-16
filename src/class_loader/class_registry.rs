use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

use crate::class_loader::class_file::ClassFile;
use crate::class_loader::raw_data::RawByteBuffer;

pub struct ClassRegistry {
    pub class_path_folder: String,
}

impl ClassRegistry {
    pub fn new(class_path_folder: String) -> Self {
        let mut real_classpath_folder = String::new();

        let current_dir = &env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        if class_path_folder == "." {
            real_classpath_folder.push_str(current_dir);
        } else {
            real_classpath_folder.push_str(current_dir);
            real_classpath_folder.push('/');
            real_classpath_folder.push_str(&class_path_folder);
        }

        Self {
            class_path_folder: real_classpath_folder,
        }
    }

    pub fn load_class(&self, class_name: &str) -> Result<ClassFile, Error> {
        let class_path_folder = &self.class_path_folder;

        let full_class_path = format!(
            "{}/{}.class",
            class_path_folder,
            Self::class_name_to_path(class_name)
        );

        println!(
            "Loading class '{}' into JVM with path: '{}'",
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
