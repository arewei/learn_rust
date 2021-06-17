use error_chain::error_chain;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use std::net::IpAddr;
use std::str;

error_chain! {
    foreign_links {
            Utf8(std::str::Utf8Error);
            AddrParse(std::net::AddrParseError);
    }
}

fn main() -> Result<()> {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

    let bytes = b"2001:db8::1";
    let s = str::from_utf8(bytes)?;
    let addr: IpAddr = s.parse()?;
    println!("{:?}", addr);

    Ok(())
}
