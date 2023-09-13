#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

mod class_info;
use class_info::class_file::ClassFile;
use class_info::raw_data::RawByteBuffer;

const HEX_DIGITS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

fn main() {
    const FILE_PATH: &str =
        // "/Users/mstepan/repo/app-java17-template/target/classes/com/max/app17/MyInterface.class";
        "/Users/mstepan/repo/app-java17-template/target/classes/com/max/app17/Main.class";

    let mut class_file_buf = BufReader::new(File::open(FILE_PATH).expect("Can't open file"));
    let mut buf = Vec::new();

    class_file_buf
        .read_to_end(&mut buf)
        .expect("Can't read file into memory");

    let mut raw_file_data = RawByteBuffer {
        cursor: 0,
        data: buf,
    };

    let main_class = ClassFile::new(&mut raw_file_data).expect("Can't parse class file");

    println!("{:#?}", main_class);

    println!("JVM exited successfully");
}

fn to_hex(single_byte: u8) -> String {
    let hex1 = HEX_DIGITS[(single_byte & 0xF) as usize];
    let hex2 = HEX_DIGITS[((single_byte >> 4) & 0xF) as usize];

    let mut hex_str = String::with_capacity(4);

    hex_str.push_str(&String::from(hex2));
    hex_str.push_str(&String::from(hex1));

    hex_str
}
