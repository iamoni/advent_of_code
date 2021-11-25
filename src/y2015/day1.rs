use super::read_input;

pub fn get_result() {
    let input = &read_input(1)[0];
    let charlist = input.chars();

    let mut floor = 0;
    let mut position = 0;

    let mut first_basement_position = 0;

    for i in charlist {
        position += 1;

        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("unexpected character! {}", &i),
        }

        if first_basement_position == 0 {
            if floor == -1 {
                first_basement_position = position;
            }
        }
    }

    let day1 = (floor, first_basement_position);

    println!(
        "2015 Day 1:\npart 1 result: {}\npart 2 result: {}\n",
        day1.0, day1.1
    );
}
