use std::{env, fs};
use std::fs::File;
use std::io::Write;

pub fn get_content() -> String {
    return fs::read_to_string(&format!("{}.txt", env::args().nth(0).unwrap())).unwrap();
}

pub fn write_result(content: &str) -> Result<(), std::io::Error> {
    let filename = env::args().nth(0).unwrap().split("/").last().unwrap().to_owned();
    let mut file = File::create(&format!("./results/{}.txt", filename))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}