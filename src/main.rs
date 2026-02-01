mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let days_impl = 8;
    let args = std::env::args().nth(1);
    if let Some(day) = args {
        let day: u32 = day.parse().unwrap();
        if day > 0 && day <= days_impl {
            println!("Running day {day}");
            run_day(day);
        }
    } else {
        println!("Running all days");
        for day in 1..=days_impl {
            print!("Day {day}:");
            run_day(day);
            println!();
        }
    }
}

fn run_day(day: u32) {
    match day {
        1 => day1::exercise(),
        2 => day2::exercise(),
        3 => day3::exercise(),
        4 => day4::exercise(),
        5 => day5::exercise(),
        6 => day6::exercise(),
        7 => day7::exercise(),
        8 => day8::exercise(),
        _ => unimplemented!(),
    }
}
