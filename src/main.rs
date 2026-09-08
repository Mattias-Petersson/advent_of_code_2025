mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    if false {
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
    run_day_nine_testing();
}

fn run_day_nine_testing() {
    let input = day9::read_input().unwrap();
    let large = day9::largest_area_between_all_points(&input);
    println!("Largest: {large}");
    let input_new = day9::add_inbetween_points(&input);
    println!("{:?}", input_new);
}
