use std::error::Error;
use regex::Regex;
use advent_of_code_24::utils::{get_content, write_result};

fn main() -> Result<(), Box<dyn Error>> {
    let content = get_content();
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)")?;
    let mut sum = 0;
    for capture in re.captures_iter(content.as_str()) {
        let mut mul_str = capture.get(0).unwrap().as_str().to_string();
        mul_str = mul_str.replace("mul(", "");
        mul_str = mul_str.replace(")", "");
        let numbers = mul_str.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sum += numbers.get(0).unwrap() * numbers.get(1).unwrap();
    }
    write_result(sum.to_string().as_str()).expect("Failed to write result");
    Ok(())
}