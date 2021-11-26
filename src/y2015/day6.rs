use super::read_input;
use regex::Regex;

pub fn get_result() {
    let input = read_input(6);

    part1(&input);
}

fn part2(input: &Vec<String>) {
    let mut grid = create_grid();
    assert_eq!(grid.len(), 1_000_000);
    let updates = create_update_list(input);

    for u in updates {
        for mut p in &mut grid {
            update_pixel(&u, &mut p);
        }
    }

    let result: i32 = grid.into_iter().map(|x| x.power).sum();
    println!("Total power: {}", result)
}

fn create_grid() -> Vec<Pixel> {
    let mut grid = Vec::new();

    for x in 0..1000 {
        for y in 0..1000 {
            grid.push(Pixel { x, y, power: 0 })
        }
    }

    grid
}

fn create_update_list(input: &Vec<String>) -> Vec<Update> {
    let re = Regex::new(r"(?P<action>turn off|toggle|turn on)\s(?P<x_start>\d+),(?P<y_start>\d+)\sthrough\s(?P<x_end>\d+),(?P<y_end>\d+)").unwrap();
    let mut updates = Vec::new();

    for i in input {
        updates.push(parse_line(i, &re))
    }

    updates
}

fn update_pixel(update: &Update, pixel: &mut Pixel) {
    if pixel.x >= update.start_x
        && pixel.x <= update.end_x
        && pixel.y >= update.start_y
        && pixel.y <= update.end_y
    {
        match update.action {
            Action::turn_on => pixel.turn_on(),
            Action::turn_off => pixel.turn_off(),
            Action::toggle => pixel.toggle(),
        }
    }
}

fn parse_line(line: &str, re: &Regex) -> Update {
    let captures = re.captures(line).expect("regex did not capture anything!");

    let action = match &captures["action"] {
        "turn off" => Action::turn_off,
        "turn on" => Action::turn_on,
        "toggle" => Action::toggle,
        _ => panic!("Could not match the action string!"),
    };

    Update {
        start_x: captures["x_start"]
            .parse::<i32>()
            .expect("Could not parse coordinate to i32!"),
        start_y: captures["y_start"]
            .parse::<i32>()
            .expect("Could not parse coordinate to i32!"),
        end_x: captures["x_end"]
            .parse::<i32>()
            .expect("Could not parse coordinate to i32!"),
        end_y: captures["y_end"]
            .parse::<i32>()
            .expect("Could not parse coordinate to i32!"),
        action: action,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Update {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    action: Action,
}
#[derive(Debug, PartialEq, Eq)]
enum Action {
    turn_on,
    turn_off,
    toggle,
}

#[derive(Debug, PartialEq, Eq)]
struct Pixel {
    x: i32,
    y: i32,
    power: i32,
}

impl Pixel {
    fn turn_on(&mut self) {
        self.power += 1;
    }

    fn turn_off(&mut self) {
        if self.power > 0 {
            self.power -= 1
        } else {
            self.power = 0
        }
    }

    fn toggle(&mut self) {
        self.power += 2;
    }

    fn new() -> Pixel {
        Pixel {
            x: 0,
            y: 0,
            power: 0,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn check_parsing() {
//         let test_line = "turn off 660,55 through 986,197";
//         let test_struct = Update {
//             start_x: 660,
//             start_y: 55,
//             end_x: 986,
//             end_y: 197,
//             action: Action::turn_off,
//         };
//         let re = Regex::new(r"(?P<action>turn off|toggle|turn on)\s(?P<x_start>\d+),(?P<y_start>\d+)\sthrough\s(?P<x_end>\d+),(?P<y_end>\d+)").unwrap();

//         assert_eq!(parse_line(test_line, &re), test_struct)
//     }
// }

// PART 1
// BEFORE CHANGES FOR PART 2

// fn create_grid() -> Vec<Pixel> {
//     let mut grid = Vec::new();

//     for x in 0..1000 {
//         for y in 0..1000 {
//             grid.push(Pixel {
//                 x,
//                 y,
//                 on_state: false,
//             })
//         }
//     }

//     grid
// }

// fn create_update_list(input: &Vec<String>) -> Vec<Update> {
//     let re = Regex::new(r"(?P<action>turn off|toggle|turn on)\s(?P<x_start>\d+),(?P<y_start>\d+)\sthrough\s(?P<x_end>\d+),(?P<y_end>\d+)").unwrap();
//     let mut updates = Vec::new();

//     for i in input {
//         updates.push(parse_line(i, &re))
//     }

//     updates
// }

// fn update_pixel(update: &Update, pixel: &mut Pixel) {
//     if pixel.x >= update.start_x
//         && pixel.x <= update.end_x
//         && pixel.y >= update.start_y
//         && pixel.y <= update.end_y
//     {
//         match update.action {
//             Action::turn_on => pixel.turn_on(),
//             Action::turn_off => pixel.turn_off(),
//             Action::toggle => pixel.toggle(),
//         }
//     }
// }

// fn parse_line(line: &str, re: &Regex) -> Update {
//     let captures = re.captures(line).expect("regex did not capture anything!");

//     let action = match &captures["action"] {
//         "turn off" => Action::turn_off,
//         "turn on" => Action::turn_on,
//         "toggle" => Action::toggle,
//         _ => panic!("Could not match the action string!"),
//     };

//     Update {
//         start_x: captures["x_start"]
//             .parse::<i32>()
//             .expect("Could not parse coordinate to i32!"),
//         start_y: captures["y_start"]
//             .parse::<i32>()
//             .expect("Could not parse coordinate to i32!"),
//         end_x: captures["x_end"]
//             .parse::<i32>()
//             .expect("Could not parse coordinate to i32!"),
//         end_y: captures["y_end"]
//             .parse::<i32>()
//             .expect("Could not parse coordinate to i32!"),
//         action: action,
//     }
// }

// #[derive(Debug, PartialEq, Eq)]
// struct Update {
//     start_x: i32,
//     start_y: i32,
//     end_x: i32,
//     end_y: i32,
//     action: Action,
// }
// #[derive(Debug, PartialEq, Eq)]
// enum Action {
//     turn_on,
//     turn_off,
//     toggle,
// }

// #[derive(Debug, PartialEq, Eq)]
// struct Pixel {
//     x: i32,
//     y: i32,
//     on_state: bool,
// }

// impl Pixel {
//     fn turn_on(&mut self) {
//         self.on_state = true;
//     }

//     fn turn_off(&mut self) {
//         self.on_state = false;
//     }

//     fn toggle(&mut self) {
//         self.on_state = !self.on_state;
//     }

//     fn new() -> Pixel {
//         Pixel {
//             x: 0,
//             y: 0,
//             on_state: false,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn check_parsing() {
//         let test_line = "turn off 660,55 through 986,197";
//         let test_struct = Update {
//             start_x: 660,
//             start_y: 55,
//             end_x: 986,
//             end_y: 197,
//             action: Action::turn_off,
//         };
//         let re = Regex::new(r"(?P<action>turn off|toggle|turn on)\s(?P<x_start>\d+),(?P<y_start>\d+)\sthrough\s(?P<x_end>\d+),(?P<y_end>\d+)").unwrap();

//         assert_eq!(parse_line(test_line, &re), test_struct)
//     }
// }
