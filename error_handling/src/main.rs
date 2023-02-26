use std::fs::File;
use std::io::{self, ErrorKind, Read};


#[allow(unused)]
fn main() {
    // panic!("error!");  // calling `panic!`

    // let v = vec![1, 2, 3];
    // v[99];  // panicked at 'index out of bounds

    // matching on different errors
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };

    // let greeting_file = File::open("hi.txt").unwrap();

    // let greeting_file = File::open("hi.txt")
    //     .expect("hi.txt should be included in this project");

    let username = read_username_from_file("hello.txt").unwrap_or_else(|error| {
            panic!("Problem opening the file: {:?}", error);
    });
    println!("username: {}", username);

    println!("{}", last_char_of_first_line("Tom and Jerry\nare good friends!").unwrap_or('\0'));
    println!("{}", last_char_of_first_line("汤姆和杰瑞\n是一对好朋友！").unwrap_or('\0'));
}

#[allow(dead_code)]
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);
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

#[allow(dead_code)]
fn read_username_from_file_v2(path: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(dead_code)]
fn read_username_from_file_v3(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
