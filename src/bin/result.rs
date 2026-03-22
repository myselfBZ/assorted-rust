use std::{fs::{self}, io::{ErrorKind, Error}};

fn main() {
    let file = fs::File::open("hello.txt").unwrap();
}


fn unwrap_or_else() {
    let file = fs::File::open("hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            panic!("file not found")
        } else {
            panic!("sth else went wrong")
        }
    });
}

fn expect() {
    let file = fs::File::open("hello.txt").expect("this is expected");
}

fn propagate_syntax_sugar() -> Result<fs::File, Error> {
    let file = fs::File::open("hello.txt")?;

    Ok(file)
}
