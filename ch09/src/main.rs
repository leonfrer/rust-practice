fn main() {
    recoverable();
    shortcuts_panic();
    read_username_from_file().unwrap();
}

// the ? operator can only be used in functions that return Result
// use std::error::Error;
// fn main() -> Result<(), Box<dyn Error>> {
//     let path = "hello.txt";
//     let
//     _f = File::open(path)?;
//     Ok(())
// }

use core::panic;
use std::fs;
use std::io;
use std::{fs::File, io::ErrorKind};

fn recoverable() {
    let path = "hello.txt";
    let _f = File::open(path);
    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // or this equivalent
    let _f = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}

fn shortcuts_panic() {
    let path = "hello.txt";
    let _f = File::open(path).expect("Fail to open file");
    let _f = File::open(path).unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let path = "hello.txt";
    // let mut f = File::open(path)?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    // or
    // File::open(path)?.read_to_string(&mut s)?;
    // or
    fs::read_to_string(path)
}
