use std::{error::Error, fs::File, io::BufReader};

pub fn read_input(day: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(format!("src/{day}/input.txt"))?;
    Ok(BufReader::new(file))
}
