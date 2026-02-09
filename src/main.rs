mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

const DAYS: &[fn()] = &[
    day1::exercise,
    day2::exercise,
    day3::exercise,
    day4::exercise,
    day5::exercise,
    day6::exercise,
    day7::exercise,
    day8::exercise,
];

fn main() {
    let args = std::env::args().nth(1);
    if let Some(day) = args {
        let day: usize = day.parse().unwrap();
        if day > 0 && day <= DAYS.len() {
            println!("Running day {day}");
            DAYS[day - 1]();
        }
    } else {
        println!("Running all days");
        for (i, day_fn) in DAYS.iter().enumerate() {
            println!("Day {}:", i + 1);
            day_fn();
            println!();
        }
    }
}
