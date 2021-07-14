extern crate error_chain;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use error_chain::error_chain;
use glob::glob;

// use walkdir::WalkDir;

pub fn is_pic(path: &str) -> bool {
    path.contains(".png") ||
        path.contains(".jpeg") || path.contains(".jpg")
}

fn find_pic(path: PathBuf) -> Result<String> {
    let input = File::open(path)?;
    let result = BufReader::new(input)
        .lines()
        .map(|e| e.unwrap_or_default().to_lowercase())
        .filter(|e| is_pic(e))
        .collect::<String>();

    Ok(result)
}

pub fn find_file() -> Result<()> {
    // WalkDir::new("/Users/huangzhiwei/WeChatProjects/annie-zheng-bao 2/pages")
    //     .follow_links(false)
    //     .into_iter()
    //     .filter_map(|entry| entry.ok())
    //     .map(|e| e.path().to_string_lossy().to_owned())
    //     .filter(|e| e.ends_with(".wxml"))
    //     .for_each(|e| println!("{}", e));

    glob("/Users/huangzhiwei/WeChatProjects/annie-zheng-bao 2/pages/**/*.wxml")?
        .filter_map(|e| e.ok())
        .map(|e| find_pic(e))
        .flatten()
        .for_each(|e| println!("{}", e));

    Ok(())
}

error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Error during IO"];
        Pattern(glob::PatternError);
    }
}