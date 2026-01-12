use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::read_input;

pub fn exercise() {
    let input = read_input("day4").unwrap();
    let mut grid = input_to_grid(input);
    let mut haybale_positions = save_positions(&grid);
    let mut count = 0;
    let size = (grid.len(), grid[0].len());

    loop {
        let removable: Vec<(usize, usize)> = haybale_positions
            .iter()
            .filter(|&&pos| check_neighbors(&grid, pos, size))
            .copied()
            .collect();

        if removable.is_empty() {
            break;
        }
        count += removable.len();
        for p in removable {
            haybale_positions.remove(&p);
            grid[p.0][p.1] = '.';
        }
    }

    println!("{}", count);
}

fn input_to_grid(input: BufReader<File>) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            let l = line.unwrap();
            l.chars().collect()
        })
        .collect()
}

fn save_positions(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                positions.insert((i, j));
            }
        }
    }
    positions
}

fn check_neighbors(grid: &Vec<Vec<char>>, position: (usize, usize), size: (usize, usize)) -> bool {
    let (curr_row, curr_col) = position;
    if grid[curr_row][curr_col] != '@' {
        return false;
    }
    let (no_rows, no_cols) = size;
    let mut neighbor_count = 0;

    let row_start = if curr_row == 0 { 0 } else { curr_row - 1 };
    let row_end = (curr_row + 1).min(no_rows - 1);
    let col_start = if curr_col == 0 { 0 } else { curr_col - 1 };
    let col_end = (curr_col + 1).min(no_cols - 1);

    for i in row_start..=row_end {
        for j in col_start..=col_end {
            if i == curr_row && j == curr_col {
                continue;
            }
            if grid[i][j] == '@' {
                neighbor_count += 1;
                if neighbor_count >= 4 {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_against_example() {
        let file = File::open(format!("src/day4/example_input.txt")).unwrap();
        let reader = BufReader::new(file);
        let grid = input_to_grid(reader);
        let positions = save_positions(&grid);
        let size = (grid.len(), grid[0].len());
        let count = positions
            .iter()
            .filter(|&&pos| check_neighbors(&grid, pos, size))
            .copied()
            .collect::<Vec<(usize, usize)>>()
            .len();
        assert_eq!(count, 13);
    }
}
