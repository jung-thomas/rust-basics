use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    let _v = vec![1, 2, 3];
    //_v[99];

    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _f2 = File::open("hello.txt").unwrap();

    let _f3 = File::open("hello.txt").expect("Failed to open hello.txt");

    let _s1 = read_username_from_file().expect("Oh 💩");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello2.txt")?.read_to_string(&mut s)?;
    Ok(s)
}