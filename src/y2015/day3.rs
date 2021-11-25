use super::read_input;
use std::collections::HashMap;

pub fn get_result() {
    let input = read_input(3)[0].clone();
    let mut houses: HashMap<Position, i32> = HashMap::new();
    let mut cursor = Position { x: 0, y: 0 };
    houses.insert(cursor, 1);

    for x in input.chars() {
        match x {
            '>' => cursor.x += 1,
            '^' => cursor.y += 1,
            '<' => cursor.x -= 1,
            'v' => cursor.y -= 1,
            _ => println!("Could not match character! {}", x),
        }

        houses.entry(cursor).and_modify(|x| *x += 1).or_insert(1);
    }

    println!(
        "Part 1: Houses that received at least 1 present: {}",
        houses.len()
    );

    let part2 = part2(input);
    println!(
        "Part 2: Houses that received at least 1 present: {}",
        &part2
    );
}

fn part2(input: String) -> usize {
    let mut houses: HashMap<Position, i32> = HashMap::new();
    let mut santa_cursor = Position { x: 0, y: 0 };
    let mut robo_cursor = Position { x: 0, y: 0 };
    let mut count = 0;

    houses.insert(santa_cursor, 2);

    for x in input.chars() {
        count += 1;
        match count % 2 {
            0 => {
                match x {
                    '>' => santa_cursor.x += 1,
                    '^' => santa_cursor.y += 1,
                    '<' => santa_cursor.x -= 1,
                    'v' => santa_cursor.y -= 1,
                    _ => println!("Could not match character! {}", x),
                }
                houses
                    .entry(santa_cursor)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
            _ => {
                match x {
                    '>' => robo_cursor.x += 1,
                    '^' => robo_cursor.y += 1,
                    '<' => robo_cursor.x -= 1,
                    'v' => robo_cursor.y -= 1,
                    _ => println!("Could not match character! {}", x),
                }
                houses
                    .entry(robo_cursor)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }
    }

    houses.len()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}
