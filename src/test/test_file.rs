extern crate error_chain;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::io::Read;
use std::path::Path;

use error_chain::error_chain;

pub fn test_file() -> Result<()> {
    let path = Path::new("/Users/huangzhiwei/Desktop/temp/error-log.txt");
    let display = path.display();

    let mut file = File::open(&path)?;
    let mut s = String::new();

    file.read_to_string(&mut s)?;

    println!("{}", s);

    Ok(())
}

pub fn test_read_write() -> Result<()> {
    let path = "/Users/huangzhiwei/Desktop/temp/error-log1.txt";
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun");

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Error during IO"];
    }
}