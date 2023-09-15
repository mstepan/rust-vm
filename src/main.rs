#![allow(dead_code)]
use std::env;

mod class_loader;
use crate::class_loader::class_registry::ClassRegistry;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Provide main class for execution");
    }

    let main_class_name = &args[1];

    let global_class_registry = ClassRegistry {};
    let maybe_main_class = global_class_registry.load_class(main_class_name);

    match maybe_main_class {
        Ok(class_file) => println!("{:#?}", class_file),
        Err(err) => panic!("Failed to load main class {}", err),
    }

    println!("JVM exited successfully");
}
