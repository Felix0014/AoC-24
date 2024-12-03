use std::fs;
use advent_of_code_24::utils::{get_content, write_result};

fn main() {
    const SEPERATOR: &str = "   ";
    let content = get_content();
    let lines: Vec<&str> = content.split("\n").collect();
    let mut first_row: Vec<i32> = Vec::with_capacity(lines.len());
    let mut second_row: Vec<i32> = Vec::with_capacity(lines.len());
    for line in lines {
        if line.split(SEPERATOR).collect::<Vec<&str>>().len() == 2 {
            let splitted = line.split(SEPERATOR).collect::<Vec<&str>>();
            first_row.push(splitted[0].parse::<i32>().unwrap());
            second_row.push(splitted[1].parse::<i32>().unwrap());
        }
    }
    first_row.sort();
    second_row.sort();
    if first_row.len() != second_row.len() {
        panic!("First row is not as long as second row");
    }
    let mut difference: Vec<i32> = Vec::with_capacity(first_row.len());
    for i in 0..first_row.len() {
        difference.push((first_row[i] - second_row[i]).abs());
    }
    println!("{}", difference.iter().sum::<i32>());
    write_result(difference.iter().sum::<i32>().to_string().as_str()).expect("could not write result to file");
}
