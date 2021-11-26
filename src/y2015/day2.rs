use super::read_input;
use std::cmp::min;

pub fn get_result() {
    let input = read_input(2);
    // let mut total_paper = 0;

    let total_paper: i32 = input
        .clone()
        .into_iter()
        .map(|x| make_elfbox(&x).calculate_paper())
        .sum();

    println!("Part 1: total paper required: {} sq ft", total_paper);

    let ribbon: i32 = input
        .into_iter()
        .map(|x| make_elfbox(&x).calculate_ribbon())
        .sum();

    println!("Part 1: total ribbon required: {} ft", ribbon);
}

#[derive(Debug)]
pub struct Elfbox {
    l: i32,
    w: i32,
    h: i32,
}

impl Elfbox {
    pub fn calculate_paper(&self) -> i32 {
        let shortest_side_area = min(min(self.l * self.w, self.l * self.h), self.w * self.h);

        let total_paper = (2 * self.l * self.w)
            + (2 * self.l * self.h)
            + (2 * self.h * self.w)
            + shortest_side_area;

        total_paper
    }

    pub fn calculate_ribbon(&self) -> i32 {
        let base_ribbon = self.l * 2 + self.w * 2;
        let extra_ribbon = self.l * self.w * self.h;

        base_ribbon + extra_ribbon
    }
}

fn make_elfbox(line: &str) -> Elfbox {
    let mut values: Vec<i32> = line
        .split('x')
        .map(|x| x.parse::<i32>().expect("Could not parse string to i32"))
        .collect::<Vec<i32>>();

    values.sort();

    if values.len() != 3 {
        panic!("Vec size is not 3! This sucks!")
    };

    Elfbox {
        l: values[0],
        w: values[1],
        h: values[2],
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_paper_1() {
        use super::*;
        let test_elfbox = Elfbox { l: 2, w: 3, h: 4 };

        assert_eq!(test_elfbox.calculate_paper(), 58)
    }

    #[test]
    fn test_calculate_paper_2() {
        use super::*;
        let test_elfbox = Elfbox { l: 1, w: 1, h: 10 };

        assert_eq!(test_elfbox.calculate_paper(), 43)
    }
}
