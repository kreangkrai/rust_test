use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_file()-> Result<String,io::Error>{
    let mut s = String::new();
    File::open("data.txt")?.read_to_string(&mut s)?;
    Ok(s)
}