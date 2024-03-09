use std::{env, path::PathBuf};

pub fn echo(arg: &str) -> () {
    if arg.contains("=") {
        let argument: &Vec<&str> = &arg.split("=").collect();
        if argument.len() == 2 {
            println!("{}", argument[1]);
        }
    } else {
        println!("{}, does not contain an argument", arg);
    }
}

pub fn ls() {
    let current_dir: PathBuf = match env::current_dir() {
        Ok(some) => some,
        Err(_) => panic!("Couldn't find current directory"),
    };
    let paths = std::fs::read_dir(&current_dir).unwrap();
    for path in paths {
        let clean_path = path.unwrap().path();
        let file_name = clean_path.file_name().unwrap();
        if file_name.to_str().unwrap().to_string().contains(".") {
            println!("file: {:?}", file_name);
        } else {
            println!("folder: {:?}", file_name);
        }
    }
}
