use super::read_input;

pub fn get_result() {
    let input = read_input(5);

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut good_strings_count = 0;

    for line in input {
        let mut good = true;
        if check_vowels(line) != true {
            good = false;
        };

        if check_double_character(line) != true {
            good = false;
        };

        if check_bad_strings(line) == true {
            good = false;
        }

        if good == true {
            good_strings_count += 1;
        }
    }

    println!("Good strings count: {}", good_strings_count);
}

fn check_vowels(line: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut vowel_count = 0;

    for i in vowels {
        if line.contains(i) {
            vowel_count += line.matches(i).count();
        }
    }

    if vowel_count >= 3 {
        true
    } else {
        false
    }
}

fn check_double_character(line: &str) -> bool {
    let line: Vec<char> = line.chars().collect();
    for i in 0..(line.len() - 1) {
        if line[i] == line[i + 1] {
            return true;
        }
    }

    false
}

fn check_bad_strings(line: &str) -> bool {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];

    for i in bad_strings {
        if line.contains(i) {
            return true;
        }
    }

    false
}

fn part2(input: &Vec<String>) {
    let mut count = 0;

    for i in input {
        if check_pair(i) && check_three(i) == true {
            count += 1;
        }
    }

    println!("part 2 - count: {}", &count);
}

fn check_pair(line: &str) -> bool {
    let mut count = 0;

    while count < line.len() - 1 {
        let pair = &line[count..count + 2];
        if line.matches(pair).count() > 1 {
            let p1 = line.find(pair).unwrap();
            let p2 = line.rfind(pair).unwrap();
            if p2 - p1 > 1 {
                return true;
            }
        }
        count += 1;
    }
    false
}

fn check_three(line: &str) -> bool {
    let mut count = 0;

    while count < line.len() - 2 {
        let pair = &line[count..count + 3];
        if pair.chars().nth(0).unwrap() == pair.chars().nth(2).unwrap() {
            return true;
        }
        count += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vowels() {
        assert_eq!(check_vowels("ugknbfddgicrmopn"), true);
        assert_eq!(check_vowels("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_consecutive_chars() {
        assert_eq!(check_double_character("juhfe23sdjssjhf"), true);
        assert_eq!(check_double_character("iuyr43233dskjf"), true);
        assert_eq!(check_double_character("uh3rdwduere"), false);
    }

    #[test]
    fn test_bad_string() {
        assert_eq!(check_bad_strings("3u4jfdksj3e8ur"), false);
        assert_eq!(check_bad_strings("kjhf34iabjf73h"), true);
    }

    #[test]
    fn test_pairs() {
        assert_eq!(check_pair("hf3eurds"), false);
        assert_eq!(check_pair("aaa"), false);
        assert_eq!(check_pair("aaaa"), true);
        assert_eq!(check_pair("ox3ij4jdjoxneff"), true);
    }

    #[test]
    fn test_threes() {
        assert_eq!(check_three("hf3eurds"), false);
        assert_eq!(check_three("aaa"), true);
        assert_eq!(check_three("oxo283"), true);
        assert_eq!(check_three("hjfhd2fdf"), true);
    }
}
