use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::BufRead,
};

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
    println!("Splitters hit (part one): {}", part_one());
    println!("Timelines created (part two): {}", part_two());
}

fn part_one() -> u32 {
    let input = get_input();
    let (mut beams, splitters, size) = parse_input(&input);
    let mut total_splitters_hit = 0;
    loop {
        total_splitters_hit += step_beams(&mut beams, &splitters, &size);
        if beams.is_empty() {
            break;
        }
    }
    total_splitters_hit
}

fn part_two() -> u64 {
    let input = get_input();
    let (beams, splitters, size) = parse_input(&input);
    count_timelines(beams[0].position, &splitters, &size)
}

fn get_input() -> Vec<String> {
    let input = read_input("day7").unwrap();
    input.lines().map(Result::unwrap).collect()
}

fn parse_input(input: &[String]) -> (Vec<Beam>, HashSet<Position>, GridSize) {
    let (beams, splitters) = find_start_and_splitters(input).unwrap();
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
        if beam.is_out_of_bounds(grid_size) {
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

fn count_timelines(start: Position, splitters: &HashSet<Position>, grid: &GridSize) -> u64 {
    let mut current: HashMap<Position, u64> = HashMap::new();
    current.insert(start, 1);

    let mut completed = 0;

    while !current.is_empty() {
        let mut next: HashMap<Position, u64> = HashMap::new();

        for (pos, count) in current {
            let new = Position {
                x: pos.x,
                y: pos.y + 1,
            };

            if new.x >= grid.width || new.y >= grid.height {
                completed += count;
                continue;
            }

            if splitters.contains(&new) {
                if new.x > 0 {
                    *next
                        .entry(Position {
                            x: new.x - 1,
                            y: new.y,
                        })
                        .or_insert(0) += count;
                }

                *next
                    .entry(Position {
                        x: new.x + 1,
                        y: new.y,
                    })
                    .or_insert(0) += count;
            } else {
                *next.entry(new).or_insert(0) += count;
            }
        }

        current = next;
    }

    completed
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn setup() -> Vec<String> {
        let file = File::open("src/day7/example_input.txt").unwrap();
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
    #[test]
    fn test_quantum_timelines() {
        let input = setup();
        let (start, splitters, grid) = parse_input(&input);
        assert_eq!(count_timelines(start[0].position, &splitters, &grid), 40);
    }
}
