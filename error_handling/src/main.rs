use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem creating the file: {error:?}")
        }
    };

    // let greeting_file = File::open("hello.txt").unwrap();

    // let greeting_file = File::open("hello.txt")
        // .expect("Hello.txt should be included in this project.");

    let username = match read_username_from_file_short() {
        Ok(username) => username,
        Err(error) => panic!("Problem reading username: {error:?}"),
    };

    println!("Username from hello.txt: {username}") 
}

// Manual (Longer)
#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? Operator (Shorter)
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username  = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}