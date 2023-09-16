#![allow(dead_code)]
use core::panic;
use std::env;
use std::io::{Error, ErrorKind};

mod class_loader;
use crate::class_loader::class_registry::ClassRegistry;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip 0-argument here b/c it will be just executable name
    let maybe_launch_ctx = parse_launch_params(&args[1..]);

    match maybe_launch_ctx {
        Ok(ctx) => {
            let main_class_name = &ctx.main_class;

            let global_class_registry = ClassRegistry::new(ctx.class_path);

            let maybe_main_class = global_class_registry.load_class(main_class_name);

            match maybe_main_class {
                Ok(mut class_file) => {
                    // class_file = dbg!(class_file);

                    match class_file.main_method() {
                        Ok(main_method) => {
                            println!("'main'  found and will be executed");

                            match main_method.get_bytecode() {
                                Some(bytecode) => {
                                    println!("{:#?}", bytecode);
                                }
                                None => panic!("No bytecode for 'main' function, really strange"),
                            }
                        }
                        Err(error) => panic!("Failed with {}", error),
                    }
                }
                Err(err) => panic!("Failed to load main class {}", err),
            }

            println!("JVM exited successfully");
        }
        Err(error) => panic!("Failed with {}", error),
    }
}

fn parse_launch_params(args: &[String]) -> Result<LaunchContex, Error> {
    if args.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Can't launch java process",
        ));
    }

    // if single argument passed we assume that this is main class for execution
    // example: com.max.Hello
    if args.len() == 1 {
        return Ok(LaunchContex {
            class_path: ".".to_string(),
            main_class: args[0].to_string(),
        });
    }

    // if 3 argument passed we assume that first 2 point to the class path folder
    // example: -cp path/to/classes com.max.Hello
    if args.len() == 3 {
        return Ok(LaunchContex {
            class_path: args[1].to_string(),
            main_class: args[2].to_string(),
        });
    }

    Err(Error::new(
        ErrorKind::InvalidInput,
        format!("Can't parse command line arguments {:#?}", args),
    ))
}

struct LaunchContex {
    class_path: String,
    main_class: String,
}
