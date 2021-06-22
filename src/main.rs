extern crate error_chain;

use std::fs::File;
use std::net::IpAddr;
use std::str;

use error_chain::error_chain;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::thread_rng;
use url::Position;
use url::Url;

mod algorithms;
mod phrases;

fn main() -> Result<()> {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

    let bytes = b"2001:db8::1";
    let s = str::from_utf8(bytes)?;
    let address: IpAddr = s.parse()?;
    println!("{:?}", address);

    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("{:?}", parsed);
    println!("cleaned: {}", cleaned);

    match read_file() {
        Ok(_) => println!("No Error"),
        Err(e) => println!("{}", e),
    }

    algorithms::main1();
    phrases::greetings::hello();

    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    println!("{:?}", middle);

    Ok(())
}

fn read_file() -> Result<()> {
    let _file = File::open("unknown_file.txt")?;
    Ok(())
}

error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
        AddrParse(std::net::AddrParseError);
        UrlParse(url::ParseError);
        Io(::std::io::Error) #[doc = "Error during IO"];
    }
}
