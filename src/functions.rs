extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
use std::io::Result;
use std::io;
use std::fs::File;

pub fn get_download_link(url: &str) -> Result<String> {
    let res = reqwest::get(url).unwrap();

    let document = Document::from_read(res)?;
    let web_url = document
        .find(Name("input"))
        .filter_map(|n| n.attr("onclick")).nth(0).unwrap();
    let string_web_url: String = web_url.to_owned();

    let url_splited: Vec<String> = string_web_url.split("href=").map(|c| c.replace("'", "")).collect();
    Ok(url_splited[1].to_owned())
}

pub fn download_link(url: &str, file_name: &str, dir_name: &str) {
    let mut resp = reqwest::get(url).expect("request failed");
    let mut out = File::create(format!("{}/{}", dir_name, file_name)).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}