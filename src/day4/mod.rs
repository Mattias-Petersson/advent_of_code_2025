use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::read_input;

pub fn exercise() {
    let input = read_input("day4").unwrap();
    let grid = input_to_grid(input);
    let no_accessible = calculate_accessible(&grid);
    println!("{}", no_accessible);
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

fn calculate_accessible(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if check_neighbors(&grid, (i, j), (grid.len(), grid[0].len())) {
                count += 1;
            }
        }
    }
    count
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
        let no_accessible = calculate_accessible(&grid);
        assert_eq!(no_accessible, 13);
    }
}
