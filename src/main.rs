use std::{fs::File, str::FromStr, io::{Read, Write}, process::Command};

use cookie_store::CookieStore;
use serde::Deserialize;
use time::OffsetDateTime;
use ureq::Cookie;

const CONFIG_FILE_NAME: &str = ".aocdown";

#[derive(Debug, Deserialize)]
struct Config {
    session_cookie: String,
    year: Option<i32>,
    day: Option<u8>,
}

const TEMPLATE: &str = r##"use std::fs;

fn main() {
    let mut s = fs::read_to_string("input.txt").unwrap();
}
"##;


fn main() {
    let config_file = File::open(CONFIG_FILE_NAME).expect("Missing config file");
    let config: Config = serde_json::from_reader(config_file).unwrap();
    let time = OffsetDateTime::now_utc();
    let year = time.year();
    let day = time.day();
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        if let Some(year) = config.year {
            year
        } else {
            year
        },
        if let Some(day) = config.day {
            day
        } else {
            day
        }
    );
    let cookie = Cookie::new("session", config.session_cookie);
    let mut my_store = CookieStore::load_json(&*Vec::new()).unwrap();
    let url = url::Url::from_str(&url).unwrap();
    let cookie = cookie_store::Cookie::try_from_raw_cookie(&cookie, &url).unwrap();
    my_store.insert(cookie, &url).unwrap();
    let agent = ureq::builder()
        .cookie_store(my_store)
        .build();
    
    let resp = agent.get(url.as_str()).call().unwrap();
    let path = format!("{}/{}", year, day);
    std::fs::create_dir_all(&path).ok();
    let mut f = File::create(format!("{}/input.txt", &path)).unwrap();
    let mut v = Vec::new();
    resp.into_reader().read_to_end(&mut v).unwrap();
    f.write(&v).unwrap();

    let output = Command::new("cargo")
        .args(["init", &path, "--name", &format!("day_{}", day)])
        .output()
        .unwrap();
    let main_path = format!("{}/src/main.rs", path);
    let mut f = File::create(&main_path).unwrap();
    write!(f, "{}", TEMPLATE).unwrap();
}
