mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let days_impl = 4;
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
        _ => unimplemented!(),
    }
}
