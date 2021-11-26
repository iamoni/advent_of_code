pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(day: i32) -> Vec<String> {
    let path = format!("input/2015/day{}.txt", day);
    println!("path is: {}", path);
    let file = File::open(path).expect("Could not read input file");
    let result: Vec<String> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line!"))
        .collect();
    result
}
