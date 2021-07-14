extern crate error_chain;

use std::fs::*;
use std::io::{BufRead, BufReader};

use error_chain::error_chain;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

// 是否是指定的文件
fn is_specified_file(entry: &DirEntry, format: &str) -> bool {
    entry.file_name()
        .to_str()
        .unwrap_or("")
        .ends_with(format)
}

// 指定目录下获取文件列表
fn get_file_paths(dir: &str) -> Vec<String> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_specified_file(e, ".wxml"))
        .map(|e| String::from(e.path().to_string_lossy()))
        .collect::<Vec<String>>()
}

// 是否包含图片
fn is_contains_img(line: &str) -> bool {
    // let line = line.to_lowercase();
    // line.contains(".png") || line.contains(".jpeg") || line.contains(".jpg")
    line.to_lowercase().contains("<image")
}

// 查找
pub fn find() -> Result<()> {
    let file_path_list = get_file_paths("/Users/huangzhiwei/WeChatProjects/annie-zheng-bao 2/pages");
    for file_path in file_path_list.into_iter() {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let src_re = Regex::new(r#".+src='|"(\w+)'|".+"#)?;

        reader.lines()
            .map(|e| e.unwrap())
            .filter(|e| is_contains_img(e))
            .for_each(|e| {
                let cap = src_re.captures(e.as_str().trim()).unwrap();
                println!("{}, {}", &cap[0], e.trim());
            })
    }

    Ok(())
}

error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Error during IO"];
        Pattern(glob::PatternError);
        Reg(regex::Error);
    }
}