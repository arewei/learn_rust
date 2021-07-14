extern crate error_chain;

use std::error::*;
use std::fs::*;
use std::io::{BufRead, BufReader};

use walkdir::WalkDir;
use error_chain::error_chain;

use crate::test::is_pic;

pub fn find() -> Result<(), Box<dyn Error>> {
    WalkDir::new("/Users/huangzhiwei/WeChatProjects/annie-zheng-bao 2/pages")
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|e| e.path().to_string_lossy().to_lowercase())
        .filter(|e| e.trim().ends_with(".wxml"))
        .for_each(|e| {
            let file = File::open(e).unwrap();
            let buffered = BufReader::new(file);
            buffered.lines()
                .filter(|e| {
                    let line = e?;
                    is_pic(line)
                })
                .for_each(|e| {})
        });

    Ok(())
}

