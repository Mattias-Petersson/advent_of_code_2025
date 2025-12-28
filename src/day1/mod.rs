use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() {
    let starting_value = 50;
    let res = calc_rotation(starting_value).unwrap();
    println!("{}", res);
}

fn calc_rotation(starting_value: i32) -> Result<u32, Box<dyn Error>> {
    let mut value: i32 = starting_value;
    let mut count = 0;
    let reader = read_input()?;
    for line in reader.lines() {
        let l = line?;
        let (direction, step_str) = l.split_at(1);
        let step: i32 = step_str.parse()?;

        match direction {
            "L" => value -= step,
            "R" => value += step,
            _ => return Err("Invalid direction".into()),
        }

        if value % 100 == 0 {
            count += 1;
        }
    }

    Ok(count)
}

fn read_input() -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open("src/day1/input.txt")?;
    Ok(BufReader::new(file))
}
