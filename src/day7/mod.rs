use std::{collections::HashSet, error::Error, io::BufRead};

use advent_of_code_2025::read_input;

#[derive(Debug)]
struct Beam {
    position: Position,
}
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
struct GridSize {
    width: usize,
    height: usize,
}

impl Beam {
    fn step(&mut self) {
        self.position.y += 1;
    }
    fn is_out_of_bounds(&self, bounds: &GridSize) -> bool {
        self.position.x >= bounds.width || self.position.y >= bounds.height
    }
}

fn split(pos: Position) -> Vec<Beam> {
    let mut output = Vec::with_capacity(2);
    if pos.x > 0 {
        output.push(Beam {
            position: Position {
                x: pos.x - 1,
                y: pos.y,
            },
        });
    }
    output.push(Beam {
        position: Position {
            x: pos.x + 1,
            y: pos.y,
        },
    });
    output
}

pub fn exercise() {
    let input = get_input();
    let (mut beams, splitters, size) = parse_input(&input);
    let mut total_splitters_hit = 0;
    loop {
        total_splitters_hit += step_beams(&mut beams, &splitters, &size);
        if beams.is_empty() {
            break;
        }
    }
    println!("{}", total_splitters_hit);
}

fn get_input() -> Vec<String> {
    let input = read_input("day7").unwrap();
    input.lines().map(Result::unwrap).collect()
}

fn parse_input(input: &Vec<String>) -> (Vec<Beam>, HashSet<Position>, GridSize) {
    let (beams, splitters) = find_start_and_splitters(&input).unwrap();
    let size = GridSize {
        width: input.first().unwrap().len(),
        height: input.len(),
    };
    (beams, splitters, size)
}

fn find_start_and_splitters(
    input: &[String],
) -> Result<(Vec<Beam>, HashSet<Position>), Box<dyn Error>> {
    let mut splitters: HashSet<Position> = HashSet::new();
    let first = input
        .first()
        .ok_or("No first element")?
        .chars()
        .position(|c| c == 'S')
        .ok_or("No start pos")?;
    let beam = Beam {
        position: Position { x: first, y: 0 },
    };

    for (row, line) in input.iter().enumerate().skip(1) {
        for (col, c) in line.as_bytes().iter().enumerate() {
            if *c == b'^' {
                splitters.insert(Position { x: col, y: row });
            }
        }
    }

    Ok((vec![beam], splitters))
}

fn step_beams(beams: &mut Vec<Beam>, splitters: &HashSet<Position>, grid_size: &GridSize) -> u32 {
    let mut count_splitters_hit = 0;
    let mut new_beams = Vec::new();
    beams.retain_mut(|beam| {
        beam.step();
        if beam.is_out_of_bounds(&grid_size) {
            return false;
        }
        if splitters.contains(&beam.position) {
            count_splitters_hit += 1;
            new_beams.extend(split(beam.position));
            return false;
        }
        true
    });
    beams.append(&mut new_beams);

    let mut seen = HashSet::new();
    beams.retain(|beam| seen.insert(beam.position));
    count_splitters_hit
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn setup() -> Vec<String> {
        let file = File::open(format!("src/day7/example_input.txt")).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(Result::unwrap).collect()
    }

    #[test]
    fn test_tachyon_beams() {
        let input = setup();
        let (mut beams, splitters, size) = parse_input(&input);

        let mut count = 0;
        loop {
            count += step_beams(&mut beams, &splitters, &size);
            if beams.is_empty() {
                break;
            }
        }
        assert_eq!(count, 21)
    }
}
