use std::error::Error;
use std::io::BufRead;

use advent_of_code_2025::read_input;

pub fn exercise() {
    let starting_value = 50;
    let res = calc_rotation(starting_value).unwrap();
    println!("{}", res);
}

fn calc_rotation(starting_value: i32) -> Result<u32, Box<dyn Error>> {
    let mut value: i32 = starting_value;
    let mut count = 0;
    let reader = read_input("day1")?;
    for line in reader.lines() {
        let l = line?;
        let (direction, step_str) = l.split_at(1);
        let step = step_str.parse()?;
        move_steps(&mut value, &mut count, step, direction);
    }
    Ok(count)
}

fn move_steps(value: &mut i32, count: &mut u32, step: i32, direction: &str) {
    let prev = *value;
    match direction {
        "L" => *value -= step,
        "R" => *value += step,
        _ => unreachable!("Can't be reached"),
    }
    // If the sign swapped, 0 was hit
    if (*value < 0 && prev > 0) || (*value > 0 && prev < 0) {
        *count += 1;
    }
    // If the dial is at 0, a count is added
    if *value == 0 {
        *count += 1;
    }
    // Rotations can be >100, check how many rotations occurred
    if value.abs() >= 100 {
        let (q, r) = (*value / 100, *value % 100);
        *count += q.abs() as u32;
        *value = r;
    }
    *value = value.rem_euclid(100);
}
