use super::read_input;
use md5;
use std::str;

pub fn get_result() {
    let input = &read_input(4)[0];
    part1(input)
}

fn part1(input: &str) {
    let mut current_number: usize = 0;

    loop {
        let current_string = current_number.to_string();
        let key = String::from(input) + &current_string;
        let hash = format!("{:x}", md5::compute(&key));

        if &hash[0..=5] == "000000" {
            println!("Key is: {}", &key);
            break;
        }

        current_number += 1
    }
}
