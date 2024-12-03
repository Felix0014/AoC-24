use std::error::Error;
use advent_of_code_24::utils::{get_content, write_result};

fn main() -> Result<(), Box<dyn Error>> {
    let content = get_content();
    let lines: Vec<&str> = content.split("\n").collect();

    let mut safe_count = 0;
    for line in lines {
        let numbers_as_string = line.split(" ").collect::<Vec<&str>>();
        if numbers_as_string.len() <= 1 { continue; }
        let numbers: Vec<i32> = line.split(" ").collect::<Vec<&str>>().iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let direction = numbers[0] - numbers[1];
        let mut save = true;
        for i in 1..numbers.len() {
            if (numbers[i - 1] - numbers[i]).abs() > 3 ||
                direction * (numbers[i - 1] - numbers[i]) <= 0 {
                save = false;
                break;
            }
        }
        if save { safe_count += 1 }
    }
    write_result(safe_count.to_string().as_str()).expect("could not write result");
    Ok(())
}